# harddots

Simple automation of state (configuration, packages) management for UNIX like systems. Imagine Ansible but much simpler.

## Concepts

- single file contains the inventory and the tasks
- Zero remote dependencies: use SSH + standard Unix tools only
- Single binary, no agent installation
- Declarative tasks with imperative execution
- Fail fast with clear error messages

## Architecture

### Core Components

1. **Task Parser** - TOML task definitions
2. **SSH Executor** - Command execution over SSH
3. **Template Engine** - MiniJinja for local rendering
4. **Inventory Manager** - Simple host list with groups

### Execution Model

```
Driver Node (localhost)
  ├─> Parse tasks
  ├─> Render templates locally
  ├─> For each target host:
  │     ├─> SSH connect
  │     ├─> Execute commands
  │     └─> Transfer files via stdin
  └─> Collect results
```

## Error Handling

- SSH connection failures → retry with backoff
- Command failures → halt on error (configurable)
- Template syntax errors → fail before SSH
- Missing variables → compile-time check

## Example manifest

```
repo/
├── manifests
│   ├── dev-env.toml
│   └── os-settings.toml
├── nodes
│   ├── dev.toml
│   ├── home-lab.toml
│   └── prod.toml
└── templates
    ├── fish
    │   └── config.fish.j2
    ├── starship
    │   └── starship.toml
    └── tmux
        └── tmux.conf.j2
```

- dev-env.toml

```TOML
name = "dev-env"

[[tasks]]
type = "package"
packages = ["fish", "tmux", "starship"]

[[tasks]]
type = "template"
templates = [
  { src = "fish/config.fish.j2", dest = "~/.config/fish/config.fish" },
  { src = "tmux/tmux.conf.j2", dest = "~/.tmux.conf" },
  { src = "starship/starship.toml", dest = "~/.config/starship.toml" },
]

```
