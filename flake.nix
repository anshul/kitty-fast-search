{
  description = "Kitty Fast Search - High Performance Terminal Search Plugin";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      isDarwin = pkgs.stdenv.isDarwin;

      # Use stable Rust with additional components
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src" "rust-analyzer" "clippy" "rustfmt"];
        targets = ["x86_64-unknown-linux-gnu" "aarch64-apple-darwin" "x86_64-apple-darwin"];
      };

      darwinPackages = pkgs.lib.optionals isDarwin [
        pkgs.darwin.apple_sdk.frameworks.CoreFoundation
        pkgs.darwin.apple_sdk.frameworks.CoreServices
        pkgs.darwin.apple_sdk.frameworks.Security
        pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        pkgs.libiconv
      ];

      linuxPackages = pkgs.lib.optionals (!isDarwin) [
        pkgs.pkg-config
        pkgs.openssl
        pkgs.perf-tools
        pkgs.valgrind
      ];
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs;
          [
            # Rust toolchain
            rustToolchain
            cargo-watch
            cargo-edit
            cargo-audit
            cargo-deny
            cargo-outdated
            cargo-flamegraph
            cargo-criterion

            # Terminal and kitty
            kitty

            # Development tools
            ripgrep
            fd
            fzf
            bat
            jq
            git

            # Performance profiling
            # valgrind (Linux-only - included in linuxPackages)

            # Build tools and utilities
            just
            gnumake
            cmake
          ]
          ++ darwinPackages ++ linuxPackages;

        shellHook = ''
          echo "🔍 Kitty Fast Search Development Environment"
          echo ""
          echo "Rust: $(rustc --version)"
          echo "Cargo: $(cargo --version)"
          echo "Kitty: $(kitty --version 2>/dev/null || echo 'Not found - install kitty separately if needed')"
          echo ""
          echo "Available commands:"
          echo "  cargo new kitty-search     # Initialize Rust project"
          echo "  cargo build                # Build the project"
          echo "  cargo run                  # Run the search binary"
          echo "  cargo test                 # Run tests"
          echo "  cargo bench                # Run benchmarks"
          echo "  cargo watch -x run         # Auto-rebuild on changes"
          echo "  cargo flamegraph           # Profile performance"
          echo ""
          echo "Suggested crates for search performance:"
          echo "  - ripgrep (grep crate)     # High-performance text search"
          echo "  - memmap2                  # Memory-mapped file I/O"
          echo "  - crossterm                # Cross-platform terminal manipulation"
          echo "  - ratatui                  # Terminal UI framework"
          echo "  - tokio                    # Async runtime"
          echo "  - clap                     # Command line parsing"
          echo "  - serde                    # Serialization"
          echo "  - regex                    # Regular expressions"
          echo ""
        '';

        # Environment variables for development
        RUST_BACKTRACE = "1";
        RUST_LOG = "debug";
      };

      # Package definition for the actual plugin
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "kitty-fast-search";
        version = "0.1.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        buildInputs = darwinPackages ++ linuxPackages;

        meta = with pkgs.lib; {
          description = "High-performance search plugin for Kitty terminal";
          homepage = "https://github.com/anshul/kitty-fast-search";
          license = licenses.mit;
          maintainers = [];
        };
      };
    });
}
