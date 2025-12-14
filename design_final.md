# HardDots: Final Design Document

## Overview

**What it is:**
A single-binary SSH orchestrator that applies manifests (packages + templated config files) to multiple Linux servers. No agents, no complex DSL, no state files—just SSH commands executed in a predictable order.

**What it isn't:**

- Not a configuration management platform (Ansible/Chef/Puppet replacement for teams)
- Not a cloud orchestrator
- Not idempotent-by-design (relies on package manager idempotency)
- Not a parallel executor (runs serially for simplicity)

**Philosophy:**
Explicit is better than implicit. Simple and understandable beats clever and efficient.

---

## Core Concepts

### 1. Manifest

A TOML file describing **what** to do:

- Install packages (OS-specific lists, with common packages shared across OSes)
- Deploy files from templates
- Define variables for templating

### 2. Node File

A TOML file describing **where** to apply manifests:

- List of servers
- SSH connection details
- Per-node variable overrides

### 3. Execution Model

For each node:

1. Detect OS (`uname -s` + `cat /etc/os-release`)
2. Install packages
3. Render and copy files
4. **On any failure:** log error, skip to next node

### 4. Templates

Jinja2-style templates (via minijinja) with variables from:

- Manifest `[vars]` section (global defaults)
- Node-specific `vars` table (overrides)

---

## Configuration Format

### Manifest (`manifests/dev-env.toml`)

```toml
# Global variables available to all templates
[vars]
theme = "dark"
editor = "nvim"

# Packages to install
[packages]
common = ["git", "curl", "wget"]  # Installed on all OSes

[packages.debian]
install = ["fish", "tmux"]

[packages.alpine]
install = ["fish", "tmux"]

# Files to deploy
[[files]]
template = "templates/fish/config.fish.j2"
dest = "~/.config/fish/config.fish"
mode = "0644"  # optional, defaults to 0644

[[files]]
template = "templates/tmux/tmux.conf.j2"
dest = "~/.tmux.conf"
mode = "0644"

[[files]]
template = "templates/starship/starship.toml.j2"
dest = "~/.config/starship.toml"
```

### Node File (`nodes/home-lab.toml`)

```toml
# Multiple nodes in one file
[[node]]
host = "pi1.local"
user = "admin"
# Optional: port = 2222
# Optional: identity_file = "~/.ssh/id_ed25519_homelab"

[[node]]
host = "192.168.1.50"
user = "root"
port = 2222

# Node with variable overrides
[[node]]
host = "desktop.local"
user = "john"
vars = { theme = "light" }  # Overrides manifest defaults

[[node]]
host = "server1.example.com"
user = "deploy"
identity_file = "~/.ssh/deploy_key"
vars = { theme = "solarized" }
```

### Template Example (`templates/fish/config.fish.j2`)

```fish
set -gx TERM xterm-256color

starship init fish | source
mise activate fish | source
```

---

## File Structure

```
harddots-repo/
├── manifests/
│   └── dev-env.toml      # Development tools manifest
├── nodes/
│   ├── home-lab.toml     # Home servers
│   ├── vps.toml          # Cloud VPS instances
│   └── dev-boxes.toml    # Development machines
└── templates/
    ├── fish/
    │   └── config.fish.j2
    ├── starship/
    │   └── starship.toml
    └── tmux/
        └── tmux.conf.j2
```

---

## Usage Examples

### 1. Apply manifest to all nodes in a file

```bash
harddots apply -m manifests/dev-env.toml -n nodes/home-lab.toml
```

Output:

```
[pi1.local] Detecting OS... ubuntu (22.04)
[pi1.local] Installing packages: git curl wget fish tmux
[pi1.local] Rendering template: templates/fish/config.fish.j2
[pi1.local] Copying to pi1.local:~/.config/fish/config.fish (mode: 0644)
[pi1.local] Rendering template: templates/tmux/tmux.conf.j2
[pi1.local] Copying to pi1.local:~/.tmux.conf (mode: 0644)
[pi1.local] ✓ Success

[192.168.1.50] Detecting OS... alpine (3.18)
[192.168.1.50] Installing packages: git curl wget fish tmux
...
```

### 2. Apply to a single host (ad-hoc)

```bash
harddots apply -m manifests/dev-env.toml -H admin@pi1.local
```

### 3. Apply to a single host with SSH options

```bash
harddots apply -m manifests/dev-env.toml -H admin@pi1.local -p 2222 -i ~/.ssh/custom_key
```

### 4. Verbose mode (show exact SSH commands)

```bash
harddots apply -m manifests/dev-env.toml -n nodes/home-lab.toml -v
```

Output includes:

```
[pi1.local] Running: ssh -p 22 admin@pi1.local 'uname -s'
[pi1.local] Running: ssh -p 22 admin@pi1.local 'cat /etc/os-release'
[pi1.local] Running: ssh -p 22 admin@pi1.local 'sudo apt-get install -y git curl wget fish tmux'
[pi1.local] Running: scp -P 22 /tmp/harddots-xxxxx admin@pi1.local:/home/admin/.config/fish/config.fish
[pi1.local] Running: ssh -p 22 admin@pi1.local 'chmod 0644 /home/admin/.config/fish/config.fish'
...
```

### 5. Check what would be applied (future: dry-run)

```bash
# For v1.0, skip dry-run. Add in v1.1 if needed.
```

---

## CLI Interface

```bash
harddots apply [OPTIONS]

OPTIONS:
    -m, --manifest <FILE>       Path to manifest file (required)
    -n, --nodes <FILE>          Path to nodes file
    -H, --host <USER@HOST>      Single host (alternative to --nodes)
    -p, --port <PORT>           SSH port (only with --host)
    -i, --identity <FILE>       SSH identity file (only with --host)
    -v, --verbose               Show executed commands
    -h, --help                  Print help
```

**Validation rules:**

- Must provide either `--nodes` or `--host` (not both)
- `--port` and `--identity` only valid with `--host`
- Manifest file must exist and be valid TOML
- Template paths in manifest are resolved relative to manifest directory
- Manifest path in node file is resolved relative to node file directory

---

## Implementation Phases

### Phase 1: Core (MVP)

- [ ] Parse manifest TOML
- [ ] Parse node TOML
- [ ] SSH connection (via `std::process::Command`)
- [ ] OS detection
- [ ] Package installation (ubuntu apt, alpine apk)
- [ ] Template rendering (minijinja)
- [ ] File copying (scp)
- [ ] Error handling (abort node on failure)
- [ ] Host-prefixed output

### Phase 2: Polish

- [ ] File mode support
- [ ] Variable merging (manifest vars + node vars)
- [ ] SSH config file reading (~/.ssh/config)
- [ ] Better error messages
- [ ] Progress indicators

### Phase 3: Nice-to-Have

- [ ] Parallel execution (optional flag)
- [ ] Dry-run mode
- [ ] macOS support
- [ ] Summary report at end

---

## Key Design Decisions

| Decision                        | Rationale                                     |
| ------------------------------- | --------------------------------------------- |
| **Always copy files**           | Simpler than checksumming; dotfiles are small |
| **No package existence checks** | Package managers handle idempotency           |
| **OS auto-detection**           | Eliminates manual configuration               |
| **Serial execution**            | Easier to debug; parallel can come later      |
| **Abort node on failure**       | Prevents partial/broken state                 |
| **Continue to next node**       | One broken server shouldn't stop the rest     |
| **Direct SSH/SCP**              | No libraries = easier to understand           |
| **Multiple nodes per file**     | Natural grouping for personal infra           |
| **OS-specific sections**        | One manifest works everywhere                 |
| **Node-level var overrides**    | Flexibility without extra files               |

---

## Error Handling Strategy

```
For each node:
    Try:
        1. Detect OS
        2. Install packages (stop on failure)
        3. Copy files (stop on failure, set permissions)
        Print: "[host] ✓ Success"
    Catch:
        Print: "[host] ✗ Failed: <error message>"
        Continue to next node

Exit code:
    0 if all nodes succeeded
    1 if any node failed
```

---

## Path Resolution Rules

1. **Template paths in manifest**: Relative to manifest file directory

   ```toml
   # In manifests/dev-env.toml
   template = "templates/fish/config.fish.j2"
   # Resolves to: manifests/../templates/fish/config.fish.j2
   ```

2. **Manifest path in node file**: Relative to node file directory

   ```toml
   # In nodes/home-lab.toml
   manifest = "manifests/dev-env.toml"
   # Resolves to: nodes/../manifests/dev-env.toml
   ```

3. **Destination paths on remote**: Relative to user's home (`~` expanded by shell)
   ```toml
   dest = "~/.config/fish/config.fish"
   # Expands on remote to: /home/username/.config/fish/config.fish
   ```

---

## OS Detection Logic

- using the Rust library os-release
- falling back to somthing else if it is not applicable

---

## Getting Started (For Implementer)

1. **Cargo.toml dependencies:**

   ```toml
   [dependencies]
   argh = "0"
   basic-toml = "0"
   minijinja = "2"
   thiserror = "1"
   ```

2. **Module structure:**

   ```
   src/
   ├── main.rs           # CLI parsing
   ├── manifest.rs       # Manifest TOML parsing
   ├── node.rs           # Node TOML parsing
   ├── executor.rs       # SSH/SCP command execution
   ├── template.rs       # Minijinja rendering
   └── error.rs          # Error types
   ```

3. **Start with:**
   - Parse manifest TOML into structs
   - Parse node TOML into structs
   - Implement SSH command execution
   - Add OS detection
   - Wire it all together

---
