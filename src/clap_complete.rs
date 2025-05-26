use clap_complete::{generate, shells::Bash, shells::Fish, shells::Zsh};

fn generate_completions(shell: &str) {
    let mut cmd = Cli::command();

    match shell {
        "bash" => generate(Bash, &mut cmd, "harddots", &mut std::io::stdout()),
        "fish" => generate(Fish, &mut cmd, "harddots", &mut std::io::stdout()),
        "zsh" => generate(Zsh, &mut cmd, "harddots", &mut std::io::stdout()),
        _ => error!("Unsupported shell: {}", shell),
    }
}
