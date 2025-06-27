# Kitty Fast Search Development Guide

## Repository Overview

High-performance terminal search plugin for Kitty terminal emulator, built in Rust for optimal performance with large buffers and continuous output.

### Project Structure

```
kitty-fast-search/
├── src/                  # Rust source code
│   ├── main.rs          # Entry point and CLI
│   ├── search/          # Search engine implementation
│   ├── ui/              # Terminal UI components
│   └── kitty/           # Kitty integration layer
├── tests/               # Integration and unit tests
├── benches/             # Performance benchmarks
├── flake.nix            # Nix development environment
├── Cargo.toml           # Rust dependencies
└── README.md            # Project documentation
```

## Development Environment

### Setup

```bash
# Automatic with direnv
direnv allow

# Manual activation
nix develop

# Initialize Rust project
cargo new kitty-search --bin
```

### Available Tools

- **Rust Stable** - Latest stable toolchain with rust-analyzer
- **Performance Tools** - flamegraph, criterion, perf-tools
- **Terminal Tools** - kitty, crossterm, ratatui
- **Development Tools** - cargo-watch, cargo-edit, cargo-audit

## Architecture

### Core Components

- **Search Engine**: High-performance text search using `grep` crate
- **Terminal UI**: Floating overlay using `crossterm` + `ratatui`
- **Kitty Integration**: Remote control API communication
- **Buffer Management**: Memory-mapped I/O with `memmap2`

### Performance Goals

- Sub-100ms search on 1M+ line buffers
- Non-blocking live search as you type
- Efficient memory usage for GB-sized logs
- Zero terminal interruption during search

## Recommended Dependencies

```toml
[dependencies]
grep = "0.2"              # ripgrep's search engine
memmap2 = "0.9"           # Memory-mapped file I/O
crossterm = "0.27"        # Cross-platform terminal
ratatui = "0.24"          # Terminal UI framework
tokio = "1.0"             # Async runtime
clap = "4.0"              # CLI parsing
serde = "1.0"             # Serialization
regex = "1.0"             # Pattern matching
anyhow = "1.0"            # Error handling
```

## Development Workflow

```bash
# Build and run
cargo build --release
cargo run

# Watch for changes
cargo watch -x run

# Run tests
cargo test

# Benchmark performance
cargo bench

# Profile performance
cargo flamegraph -- --your-args
```

## Testing

### Test Categories

- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end search scenarios
- **Performance Tests**: Benchmark with large buffers
- **Kitty Integration**: Remote control API testing

### Running Tests

```bash
# All tests
cargo test

# Performance benchmarks
cargo bench

# With coverage
cargo tarpaulin --out html
```

## Commit Format

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <description>

[optional body]
[optional footer]
```

### Types

- `feat:` - New feature
- `fix:` - Bug fix
- `perf:` - Performance improvement
- `docs:` - Documentation
- `test:` - Tests
- `build:` - Build system
- `chore:` - Maintenance

### Examples

```bash
feat: implement floating search overlay
perf: optimize buffer scanning with SIMD
fix: handle terminal resize during search
test: add large buffer performance benchmarks
```

## Code Standards

- Follow Rust standard formatting (`rustfmt`)
- Use `clippy` for linting
- Comprehensive error handling
- Document public APIs
- Write performance-focused code
- Profile before optimizing

## Performance Guidelines

### Optimization Priorities

1. **Memory Efficiency**: Use memory mapping for large buffers
2. **Search Speed**: Leverage SIMD and optimized algorithms
3. **UI Responsiveness**: Non-blocking async operations
4. **Terminal Integration**: Minimal kitty API overhead

### Profiling Tools

```bash
# CPU profiling
cargo flamegraph

# Memory profiling
valgrind --tool=massif ./target/release/kitty-search

# Benchmark tracking
cargo criterion
```

## Troubleshooting

### Common Issues

- **Build failures**: Check Rust version and dependencies
- **Performance**: Profile with flamegraph
- **Kitty integration**: Verify remote control enabled
- **UI rendering**: Test terminal compatibility

### Debug Tips

- Use `RUST_LOG=debug` for verbose logging
- Test with various terminal sizes
- Profile with realistic buffer sizes
- Check kitty remote control permissions

## Architecture Decisions

### Search Engine Choice

- **ripgrep crate**: Battle-tested, SIMD-optimized
- **Custom Boyer-Moore**: For specific use cases
- **Regex engine**: Rust regex crate for patterns

### UI Framework

- **ratatui**: Rich terminal UI components
- **crossterm**: Cross-platform compatibility
- **Custom**: Minimal overhead for simple overlay

### Async Strategy

- **tokio**: Full async runtime for I/O
- **async-std**: Alternative async runtime
- **Blocking**: Dedicated thread pool for search

## Resources

- [Kitty Remote Control](https://sw.kovidgoyal.net/kitty/remote-control/)
- [ripgrep Implementation](https://github.com/BurntSushi/ripgrep)
- [ratatui Examples](https://github.com/ratatui-org/ratatui)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)