# Agent Instructions for harddots

## Build & Development Commands

```bash
# Build debug binary
mise run build-dev

# Build release binary  
mise run build-rel

# Check compilation without building
mise run check

# Run linter
mise run lint

# Run a single test (when tests exist)
cargo test test_name
```

## Code Style Guidelines

### Rust
- Use standard Rust naming: `snake_case` for functions/variables, `PascalCase` for types
- Prefer `Result<T, E>` for error handling, avoid `unwrap()` in production code
- Use `thiserror` for custom error types
- Keep `main.rs` minimal - extract logic to modules in `src/lib.rs`

### Project-Specific Conventions
- **TOML configs**: Use kebab-case for field names, descriptive task names
- **SSH execution**: Always validate commands before remote execution
- **Template rendering**: Use MiniJinja, render locally before SSH transfer
- **Error handling**: Fail fast with clear messages, implement retry with backoff for SSH
- **File structure**: Manifests in `repo/manifests/`, nodes in `repo/nodes/`, templates in `repo/templates/`
