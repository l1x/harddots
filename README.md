# harddots

Keeping my apps installed and configs in sync across unix systems

A personalized dotfile manager with a focus on idempotent deployment across different Unix-like systems. 

Let's break down the design and the commands.

### **Core Concepts**

You've got a good foundation with your **Host** and **Config** concepts.

* **Host:**  
  * type: This is crucial for package management and potentially path differences.
   I have:
    * Linux / Debian
    * Linux / Alpine
    * MacOS  
  * root_shell_command: sudo, doas, or potentially others. This will be used for installing packages.  
* **Config:**  
  * path: The relative path to the configuration file on the target system (e.g., ~/.config/starship.toml).  
  * application: The associated application (e.g., starship). This helps in knowing what package to install.  
  * state: na (not applicable/not managed), installed (application is installed), configured (config file is linked). This is good for tracking.  
  * source_git_path: The relative path to this config file within your Git repository (e.g., starship/starship.toml).

### **Configuration File (e.g., harddots.toml)**

This central TOML file will define all the applications and their configurations you want to manage.

harddots.toml:

```toml
# Global settings (optional)  
git_repo = "git@github.com:yourusername/dotfiles.git"

[[applications]]  
name = "starship"  
target_path = "~/.config/starship.toml"  
source_git_path = "starship/starship.toml"  
[applications.packages]
macos = "starship"  
debian = "starship"  
alpine = "starship"

[[applications]]  
name = "nvim"  
target_path = "~/.config/nvim/init.lua" # Or init.vim  
source_git_path = "nvim/init.lua"  
# Example of an app that might not be in default Alpine repos, or needs a specific install method  
[applications.packages]  
macos = "neovim"  
debian = "neovim"  
# alpine = "neovim" # If available, otherwise you might need custom install scripts

[[applications]]  
name = "tmux"  
target_path = "~/.tmux.conf"  
source_git_path = "tmux/tmux.conf"  
[applications.packages]  
macos = "tmux"  
debian = "tmux"  
alpine = "tmux"
```

### **Commands**

Here are some commands based on your requirements:

1. **harddots init**  
   * **Action:** Clones your dotfiles Git repository to a local cache (e.g., ~/.cache/harddots/dotfiles or ~/.local/share/harddots/dotfiles).  
   * **Purpose:** Ensures the latest configurations are available locally before any operations.  
2. **harddots deploy <application_name | all>**  
   * **Action:**  
     1. **Identify Host Type:** Determine if it's MacOS, Debian, Alpine, etc.  
     2. **Check/Install Application:**  
        * If the application_name (or all applications) is not installed, use the appropriate package manager (brew, apt, apk) and the configured root_shell_command (sudo, doas) to install it.  
        * Example: On MacOS for starship, it would run brew install starship. On Debian, sudo apt install starship -y.  
     3. **Fetch Latest Configs:** Run git pull in the local dotfiles cache.  
     4. **Link Configuration:**  
        * Resolve the target_path (expanding ~).  
        * Ensure the parent directory for target_path exists (e.g., ~/.config/).  
        * Create a **hardlink** from the corresponding source_git_path in your local Git cache to the target_path.  
          * ln /path/to/local/git/cache/starship/starship.toml ~/.config/starship.toml  
        * **Important:** Hardlinks require the source and destination to be on the same filesystem. This is usually fine for user config files. If the local Git cache is on a different partition than ~/.config, this could be an issue, but it's rare for typical setups.  
     5. **Update State:** Mark the application as installed and configured.  
   * **Idempotency:** If the application is already installed and the config is already correctly hardlinked, the command should ideally do nothing or just verify.  
3. **harddots update**  
   * **Action:**  
     1. Pulls the latest changes from your remote Git repository into the local cache.  
     2. Iterates through all configured applications.  
     3. Since you're using hardlinks, once the file in the Git cache is updated, the linked target_path automatically reflects these changes. There's no explicit "re-linking" step needed unless the hardlink was broken.  
     4. Perhaps it could verify that hardlinks are still in place.  
   * **Purpose:** Keep configurations in sync with the Git remote.  
4. **harddots status**  
   * **Action:**  
     1. Reads the harddots.toml.  
     2. For each application:  
        * Checks if the application package is installed.  
        * Checks if the target_path exists.  
        * Checks if the target_path is a hardlink to the expected source_git_path in the local cache (this requires comparing inodes).  
        * Reports the state (e.g., starship: installed, configured, htop: not installed).  
   * **Purpose:** Provide an overview of the current system's configuration status.  
5. **harddots add <application_name> --target-path <path> --source-git-path <git-path> [--macos-pkg <pkg>] [--debian-pkg <pkg>] [--alpine-pkg <pkg>]**  
   * **Action:** Adds a new application entry to your harddots.toml programmatically. This could also involve creating the file in your local Git cache and committing/pushing it.  
   * **Purpose:** Simplify adding new configurations to manage.  
6. **harddots remove <application_name>**  
   * **Action:**  
     1. Removes the hardlink at target_path.  
     2. Optionally, offers to uninstall the application package.  
     3. Removes the application entry from harddots.toml.  
   * **Purpose:** Stop managing an application's configuration.

### **Rust Implementation Considerations**

* **TOML Parsing:** The toml crate is excellent for this.  
* **Command Execution:**  
  * std::process::Command is the standard library way to run external commands (like apt, brew, ln, git).  
  * You'll need to carefully handle arguments, especially for sudo or doas.  
* **OS Detection:**  
  * The std::env::consts::OS can give you broad OS types ("macos", "linux").  
  * For Linux distributions (Debian, Alpine), you might need to check files like /etc/os-release. Crates like os_info or sys-info could be helpful.  
* **Filesystem Operations:**  
  * std::fs for creating directories, checking paths, creating hardlinks (std::fs::hard_link).  
  * std::path::PathBuf for path manipulation.  
  * To expand ~, you can use the dirs or shellexpand crates.  
* **Git Interaction:**  
  * You can either call the git command-line tool directly.  
  * Or use a Rust Git library like git2 for more programmatic control (though it can be more complex). For your initial needs, calling git CLI might be simpler.  
* **Error Handling:** Rust's Result type will be essential for robust error handling.  
* **Hardlink Verification:** To verify a hardlink, you'd get the metadata (specifically the inode number) of both the target file and the source file in your cache and compare them. std::fs::metadata() and then metadata.ino() (on Unix).

### **Why Hardlinks?**

You specified hardlinks. The main advantages are:

* **Atomic Updates:** When git pull updates the file in your cache, all hardlinked configurations are instantly updated.  
* **No Dangling Links:** If the source in the cache is moved or deleted (which it shouldn't be by your tool's design, but hypothetically), a softlink would break. A hardlink still points to the inode of the data.

The main disadvantage:

* **Same Filesystem:** Source and target must be on the same filesystem. Usually true for user configs within their home directory and a cache also in the home directory (e.g., ~/.cache).  
* **Directories:** You cannot hardlink directories. This is fine for individual config files.

### **Workflow Example (New System)**

1. Install Rust and build your harddots tool (or download a pre-built binary).  
2. Manually create a basic harddots.toml or have a command to generate a default one and point it to your Git repository.  
3. Run harddots init (this clones your dotfiles repo to ~/.cache/harddots/dotfiles).  
4. Run harddots deploy all:  
   * harddots reads harddots.toml.  
   * For starship:  
     * Detects OS (e.g., MacOS).  
     * Checks if starship is installed. If not, runs brew install starship.  
     * Creates ~/.config/ if it doesn't exist.  
     * Runs ln ~/.cache/harddots/dotfiles/starship/starship.toml ~/.config/starship.toml.  
   * Repeats for nvim, tmux, etc.

