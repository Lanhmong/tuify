# AGENTS.md - Developer Guide for tuify

This document provides guidelines for agentic coding agents working in this repository.

## Project Overview

**tuify** is a Rust CLI application that uses ratatui for terminal UI. It appears to be an OAuth/authentication helper (see `docs/spotify-auth-plan.md`) with utilities for PKCE flow (SHA256 hashing, base64 encoding, random string generation).

## Build, Lint, and Test Commands

### Build
```bash
cargo build          # Debug build
cargo build --release  # Release build
```

### Run
```bash
cargo run            # Run the application
cargo run --release  # Run release build
```

### Linting
```bash
cargo clippy         # Run clippy lints (recommended before committing)
cargo clippy --fix   # Auto-fix clippy warnings where possible
```

### Testing
```bash
cargo test           # Run all tests
cargo test <name>    # Run tests matching <name>
cargo test --no-run # Compile tests without running
```

Note: Currently there are 0 tests in the project. Add tests using `#[cfg(test)]` modules or in `tests/` directory.

### Formatting
```bash
cargo fmt            # Format code
cargo fmt --check    # Check formatting without modifying
```

### Other Useful Commands
```bash
cargo check          # Type-check without building
cargo doc --open     # Build and open documentation
cargo clean          # Clean build artifacts
```

## Code Style Guidelines

### General Principles
- Follow standard Rust conventions (rustfmt will handle most formatting)
- Keep lines under 100 characters when practical
- Use 4 spaces for indentation
- **Be pragmatic**: prefer simple solutions over clever ones
- Don't add unnecessary abstraction, comments, or boilerplate unless asked
- If something works and is readable, don't refactor it "just because"
- Only add complexity when there's a clear benefit

### Imports
- Use absolute paths with `crate::`, `super::`, or the crate name
- Group imports: std → external (crates) → local modules
- Use nested imports for related items: `use ratatui::{widgets::Block, Frame};`
- Use the "trait as trait" pattern: `use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};`

Example:
```rust
use std::collections::HashMap;

use color_eyre::{Result, eyre::WrapErr};
use tiny_http::{Response, Server};

mod server;
mod util;
```

### Naming Conventions
- **Functions/variables**: `snake_case` (e.g., `get_random_string`, `run_server`)
- **Types/structs/enums**: `PascalCase` (e.g., `DefaultTerminal`, `HashMap`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `POSSIBLE`)
- **Modules**: `snake_case` (e.g., `mod server;`)
- **Files**: `snake_case.rs` (e.g., `server.rs`, `util.rs`)

### Error Handling
- Use `color_eyre` for error handling (already a dependency)
- Return `Result<T, E>` for functions that can fail
- Use `.wrap_err()` or `.map_err()` for contextual error messages
- Use the `?` operator for propagating errors

Example from codebase:
```rust
pub fn run_server() -> Result<String> {
    let server = Server::http("127.0.0.1:8080")
        .map_err(|e| color_eyre::eyre::eyre!("Failed to start server: {}", e))?;

    let request = server
        .recv()
        .wrap_err("Failed to receive callback request")?;
    // ...
}
```

### Types and Generics
- Use explicit type annotations when not obvious from context
- Prefer generic bounds over trait objects where possible
- Use the `impl Trait` syntax for return types when appropriate

### Visibility
- Use `pub` only when the item needs to be public
- Keep helper functions private unless reused across modules
- Prefer module-level privacy over individual item privacy when possible

### Documentation
- Don't add doc comments unless the user explicitly asks for them
- Document public APIs with `///` doc comments only when requested
- Include examples in doc comments for important functions only if asked

### Testing
- Write tests using `#[cfg(test)]` modules within source files
- Add integration tests in `tests/` directory
- Test both success and error paths
- Use descriptive test names: `#[test] fn test_function_handles_empty_input()`

### Patterns Used in This Codebase

**Module organization:**
```rust
mod module_name;      // Declare module
use module_name::Foo; // Import from module
```

**Result aliases:** Not currently used, but can add:
```rust
type Result<T> = color_eyre::Result<T>;
```

**Unused code warnings:** Allow `#[allow(dead_code)]` for functions that will be used later.

### Dependencies

Key dependencies:
- `ratatui` - Terminal UI framework
- `color_eyre` - Improved error handling
- `crossterm` - Cross-platform terminal handling
- `tiny_http` - HTTP server
- `sha2` - SHA256 hashing
- `base64` - Base64 encoding
- `rand` - Random number generation

### Pre-commit Checklist

Before committing:
1. Run `cargo fmt`
2. Run `cargo clippy`
3. Run `cargo test`
4. Ensure no warnings (or document why they're acceptable)

### Project Structure

```
src/
  main.rs      - Entry point, TUI app setup
  server.rs    - HTTP server for OAuth callback
  util.rs      - Crypto utilities (PKCE helpers)
docs/
  spotify-auth-plan.md - Authentication flow documentation
```

### Rust Edition

The project uses Rust edition 2024 (requires nightly Rust). Ensure you're using a recent nightly toolchain.
