# AGENTS.md
# Build/Test commands
___
* `cargo build` - build the project
* `cargo test` - run tests
* `cargo build --release` - build the project in release mode
* `cargo clippy` - run clippy linter
* `cargo fmt` - format the code
# Style guidelines
___
* Use `snake_case` when possible
* use `PascalCase` for types and traits
* Prefer explicit types over type inference where it improves readability
* Use `anyhow::Result` for error handling
* Prefer `&str` over `String` for function parameters when possible
* Use `Option` and `Result` for functions that can fail or return nothing
* Prefer `PathBuf` for file paths when possible
* Use `#[derive(Debug)]` for structs and enums
* Prefer `match` statements over `if let` for complex pattern matching
* Use `const` for compile-time constants(e.g., `const MAX_HEALTH: u32 = 100;`)
* Use `static` with LazyLock for global state that requires initialization at runtime(e.g., `static CONFIG: LazyLock<Config> = LazyLock::new(|| { ... });`)
* Implement `Default` trait where applicable
* Structure modules with `mod.rs` files for organization
* Use the `todo!()` macro for unimplemented functionality
* Write comments to explain subtle things, but avoid superfluous comments. Always comment like this: `// this is an example comment, short and concise`
# Development guidelines
___
* Always run `cargo fmt` and `cargo clippy` before committing code
* Don't merge anything that fails the workflow checks
* Write tests for new, critical functionality