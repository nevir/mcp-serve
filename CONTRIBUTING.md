# Contributing to mcp-serve

Thank you for your interest in contributing to mcp-serve! This document provides guidelines and information for developers who want to contribute to the project.

## Development Setup

### Prerequisites

- **Rust**: Version 1.70 or later
- **Git**: For version control
- **A Unix-like environment**: macOS, Linux, or WSL on Windows

### Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/nevir/mcp-serve.git
   cd mcp-serve
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run the development version:**
   ```bash
   cargo run -- --help
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

## Project Structure

```
mcp-serve/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── scanner/             # File system scanning and tool discovery
│   ├── server/              # MCP HTTP server implementation
│   ├── registry/            # Tool registry and metadata management
│   ├── execution/           # Script execution engine
│   └── templates/           # Input/output template processing
├── examples/                # Example tools and configurations
├── tests/                   # Integration tests
├── docs/                    # Additional documentation
├── Cargo.toml              # Project manifest and dependencies
└── README.md               # User-facing documentation
```

## Development Workflow

### Branch Naming

Follow the pattern: `<feature-number>-<feature-name>/<task-number>-<task-name>`

Examples:
- `1-project-wireframe/5-setup-cargo-toml`
- `2-mcp-protocol/12-implement-tool-execution`

### Making Changes

1. **Create a feature branch:**
   ```bash
   git checkout -b <feature-number>-<feature-name>/<task-number>-<task-name>
   ```

2. **Make your changes following the code conventions below**

3. **Test your changes:**
   ```bash
   cargo test
   cargo clippy
   cargo fmt -- --check
   ```

4. **Commit your changes:**
   ```bash
   git add .
   git commit -m "Brief description of changes"
   ```

5. **Push and create a pull request:**
   ```bash
   git push -u origin your-branch-name
   gh pr create --title "Task Title" --body-file .github/pull_request_template.md
   ```

## Code Conventions

### Rust Style

- **Formatting**: Use `cargo fmt` to automatically format code
- **Linting**: Address all warnings from `cargo clippy`
- **Naming**: Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)

### Error Handling

- Use `Result<T, E>` for functions that can fail
- Prefer specific error types over generic strings
- Provide meaningful error messages that help users understand what went wrong

### Documentation

- Add doc comments (`///`) for public functions and types
- Include examples in doc comments where helpful
- Keep comments concise and focused on "why" rather than "what"

### Testing

- Write unit tests for individual functions and modules
- Write integration tests for end-to-end functionality
- Use descriptive test names that explain what is being tested
- Follow the pattern: `test_<functionality>_<expected_outcome>`

Example:
```rust
#[test]
fn test_template_parsing_with_optional_sections() {
    // Test implementation
}
```

## Testing Procedures

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in a specific module
cargo test scanner::tests
```

### Test Categories

1. **Unit Tests**: Test individual functions and modules in isolation
2. **Integration Tests**: Test complete workflows and feature interactions
3. **Example Tests**: Validate that examples in documentation work correctly

### Writing Tests

- Place unit tests in the same file as the code they test, in a `tests` module
- Place integration tests in the `tests/` directory
- Use meaningful test data that reflects real-world usage
- Test both success and error cases

## Building and Release

### Development Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Running Clippy and Formatting

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt

# Run linter
cargo clippy

# Run linter with all features
cargo clippy --all-features
```

## Architecture Guidelines

### Module Organization

- **scanner/**: File system operations and tool discovery logic
- **server/**: HTTP server and MCP protocol implementation
- **registry/**: Tool metadata storage and validation
- **execution/**: Secure script execution and process management
- **templates/**: Input/output template processing and marshalling

### Design Principles

- **Separation of Concerns**: Each module should have a single, well-defined responsibility
- **Error Propagation**: Use `?` operator for clean error propagation up the call stack
- **Resource Management**: Use RAII patterns for file handles, network connections, etc.
- **Security**: Validate all inputs and sanitize data passed to external processes

## Contribution Guidelines

### Pull Request Process

1. Fork the repository and create your branch from `main`
2. Make your changes following the conventions above
3. Add or update tests as needed
4. Update documentation if you're changing public APIs
5. Ensure all tests pass and code is properly formatted
6. Submit a pull request with a clear description of your changes

### Issue Reporting

When reporting bugs or requesting features:

- Use the appropriate issue template
- Provide clear reproduction steps for bugs
- Include relevant system information (OS, Rust version, etc.)
- Search existing issues to avoid duplicates

### Code Review

- Be respectful and constructive in feedback
- Focus on code quality, security, and maintainability
- Suggest specific improvements with examples when possible
- Approve PRs that meet the project standards

## Getting Help

- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and discuss ideas
- **Documentation**: Check docs/ directory for additional information

## License

By contributing to mcp-serve, you agree that your contributions will be licensed under the [Blue Oak Model License 1.0.0](LICENSE.md).