# harddots

A single-binary SSH orchestrator that applies manifests (packages + templated config files) to multiple Linux servers. No agents, no complex DSL, no state files.

## Concepts

- **Manifest**: TOML file describing what to install and deploy
- **Node file**: TOML file listing target servers
- Zero remote dependencies: SSH + standard Unix tools only
- Single binary, no agent installation
- Fail fast with clear error messages

## Execution Model

```
Driver Node (localhost)
  ├─> Parse manifest + nodes
  ├─> For each target host:
  │     ├─> SSH connect
  │     ├─> Detect OS
  │     ├─> Install packages
  │     ├─> Render templates locally
  │     └─> Copy files via scp
  └─> Collect results
```

## File Structure

```
repo/
├── manifests/
│   └── dev-env.toml
├── nodes/
│   ├── home-lab.toml
│   └── prod.toml
└── templates/
    ├── fish/
    │   └── config.fish.j2
    ├── starship/
    │   └── starship.toml
    └── tmux/
        └── tmux.conf.j2
```

## Example Manifest (`manifests/dev-env.toml`)

```toml
[vars]
theme = "dark"
editor = "nvim"

[packages]
common = ["git", "curl", "wget"]

[packages.debian]
install = ["fish", "tmux"]

[packages.alpine]
install = ["fish", "tmux"]

[[files]]
template = "templates/fish/config.fish.j2"
dest = "~/.config/fish/config.fish"

[[files]]
template = "templates/tmux/tmux.conf.j2"
dest = "~/.tmux.conf"
```

## Example Node File (`nodes/home-lab.toml`)

```toml
[[node]]
host = "pi1.local"
user = "admin"

[[node]]
host = "192.168.1.50"
user = "root"
port = 2222

[[node]]
host = "desktop.local"
user = "john"
vars = { theme = "light" }
```

## Usage

```bash
# Apply to all nodes in a file
harddots apply -m manifests/dev-env.toml -n nodes/home-lab.toml

# Apply to a single host
harddots apply -m manifests/dev-env.toml -H admin@pi1.local

# Verbose mode
harddots apply -m manifests/dev-env.toml -n nodes/home-lab.toml -v
```

## Building

```bash
mise tasks --all
```
