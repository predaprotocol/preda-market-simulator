# Contributing to Preda Market Simulator

Thank you for your interest in contributing to Preda Market Simulator! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a branch** for your changes
4. **Make your changes** and commit them
5. **Push to your fork** and submit a pull request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/predaprotocol/preda-market-simulator.git
cd preda-market-simulator

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_simulation
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure code passes linting (`cargo clippy`)
- Write tests for new functionality
- Document public APIs with doc comments
- Keep functions focused and modular

## Testing

All contributions should include appropriate tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Pull Request Process

1. **Update documentation** if you're changing public APIs
2. **Add tests** for new functionality
3. **Ensure all tests pass** (`cargo test`)
4. **Run formatting** (`cargo fmt`)
5. **Run linting** (`cargo clippy`)
6. **Update CHANGELOG.md** with your changes
7. **Submit PR** with clear description of changes

## PR Guidelines

- **Title**: Clear, concise description of changes
- **Description**: Explain what, why, and how
- **Tests**: Include test results
- **Breaking Changes**: Clearly mark any breaking changes
- **Documentation**: Update relevant docs

## Code Review

All submissions require review. We use GitHub pull requests for this purpose.

## Areas for Contribution

### High Priority

- Additional market scenarios
- More sophisticated participant behaviors
- Advanced statistical analysis
- Performance optimizations
- Documentation improvements

### Medium Priority

- Additional trading strategies
- Export formats (Parquet, Arrow)
- Visualization tools
- Benchmark suite
- Integration tests

### Low Priority

- UI/Dashboard
- Real-time streaming
- Multi-market simulations
- Machine learning integration

## Bug Reports

When filing a bug report, please include:

1. **Description**: Clear description of the bug
2. **Steps to Reproduce**: Minimal code to reproduce
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Environment**: Rust version, OS, etc.

## Feature Requests

We welcome feature requests! Please include:

1. **Use Case**: Why is this feature needed?
2. **Proposed Solution**: How should it work?
3. **Alternatives**: Other approaches considered
4. **Additional Context**: Any other relevant information

## Coding Standards

### Rust Best Practices

- Use `Result` for error handling
- Prefer `&str` over `String` for function parameters
- Use `impl Trait` for return types when appropriate
- Avoid `unwrap()` in library code
- Use `?` operator for error propagation

### Documentation

- All public items must have doc comments
- Include examples in doc comments
- Explain complex algorithms
- Document panics and errors

### Testing

- Unit tests for individual functions
- Integration tests for workflows
- Property-based tests for complex logic
- Benchmark critical paths

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

## Questions?

Feel free to open an issue for questions or join our community discussions.

## Code of Conduct

Be respectful, inclusive, and constructive. We're all here to build something great together.

---

Thank you for contributing to Preda Market Simulator! 🚀
