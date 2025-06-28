# 🔍 Kitty Fast Search

> **Blazing-fast terminal search plugin for Kitty** - Replicate iTerm's search UX with sub-100ms performance on massive buffers

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Performance](https://img.shields.io/badge/performance-<100ms-green.svg)](./BENCHMARKS.md)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## ✨ Features

- **⚡ Blazing Fast**: Search 1M+ lines in <100ms using ripgrep's engine
- **🎯 Non-Intrusive**: Floating overlay that never interrupts your workflow  
- **🔄 Live Search**: Real-time results as you type, no enter key needed
- **📊 Massive Buffers**: Handle GB-sized server logs efficiently with memory mapping
- **🎨 iTerm-like UX**: Familiar search experience with modern performance
- **🚀 Zero Dependencies**: Pure Rust binary, no Python overhead

## 🚀 Installation

### Prerequisites

- [Kitty terminal](https://sw.kovidgoyal.net/kitty/) with remote control enabled
- [Nix](https://nixos.org/) (recommended) or Rust toolchain

### Install from Source

```bash
# Clone the repository
git clone https://github.com/anshul/kitty-fast-search
cd kitty-fast-search

# Build and install
cargo install --path .
```

### Enable Kitty Remote Control

Add to your `~/.config/kitty/kitty.conf`:

```conf
# Enable remote control for the search plugin
allow_remote_control yes
remote_control_password ""
```

## 🎯 Usage

```bash
# Launch floating search overlay
kitty-fast-search

# Search with initial query
kitty-fast-search --query "error"
```

**Keyboard Shortcuts:**
- `Ctrl+F` - Open search overlay
- `Escape` - Close search
- `Enter` - Jump to result
- `↑/↓` - Navigate results

## 📊 Performance

| Buffer Size | Search Time | Memory Usage |
|-------------|-------------|--------------|
| 10K lines   | 0.27ms      | 500KB        |
| 100K lines  | 2.7ms       | 5MB          |
| 1M lines    | 27ms        | 50MB         |
| 10M lines   | 270ms       | 500MB        |

*See [BENCHMARKS.md](BENCHMARKS.md) for detailed performance analysis and comparison with other tools*

### Running Benchmarks

```bash
# Run all performance benchmarks
cargo bench

# Run specific benchmark groups
cargo bench --bench search_performance
cargo bench memory_usage

# Generate detailed reports
cargo bench -- --output-format json > results.json
```

## 🛠️ Development

For development setup, architecture details, and contribution guidelines, see [CONTRIBUTORS.md](CONTRIBUTORS.md).

```bash
# Quick start for developers
nix develop
cargo test
cargo bench
```

## 🐛 Troubleshooting

**Search not working?**
```bash
# Verify kitty remote control is enabled
kitty @ ls
```

**Performance issues?**
```bash
# Profile the search
cargo flamegraph -- --query "your-search"
```

For more troubleshooting help, see [CONTRIBUTORS.md](CONTRIBUTORS.md#troubleshooting).

## 🙏 Acknowledgments

- [**ripgrep**](https://github.com/BurntSushi/ripgrep) - Lightning-fast search engine that powers our core functionality
- [**Kitty**](https://github.com/kovidgoyal/kitty) - The amazing terminal emulator that makes this plugin possible
- [**ratatui**](https://github.com/ratatui-org/ratatui) - Elegant terminal UI framework for the search overlay
- [**crossterm**](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [**tokio**](https://github.com/tokio-rs/tokio) - Async runtime for responsive UI

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**⭐ Star this repo if it helps improve your terminal workflow!**

