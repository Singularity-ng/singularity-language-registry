{
  description = "Singularity Language Registry - Production-ready language detection for code analysis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common arguments for crane builds
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          buildInputs = with pkgs; [
            # Add runtime dependencies here if needed
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # MacOS specific dependencies
            pkgs.libiconv
            pkgs.darwin.apple_sdk.frameworks.Security
          ];

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
          ];
        };

        # Build just the cargo dependencies
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate
        singularity-language-registry = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;

          # Run tests during build
          doCheck = true;

          # Additional cargo build flags
          cargoExtraArgs = "--all-features";

          # Set stricter clippy checks
          CARGO_BUILD_RUSTFLAGS = "-D warnings";
        });

      in
      {
        # Packages that can be built
        packages = {
          default = singularity-language-registry;
          singularity-language-registry = singularity-language-registry;

          # Docker image
          docker = pkgs.dockerTools.buildLayeredImage {
            name = "singularity-language-registry";
            tag = "latest";
            contents = [ singularity-language-registry ];
            config = {
              Entrypoint = [ "${singularity-language-registry}/bin/singularity-language-registry" ];
              WorkingDir = "/";
            };
          };

          # Documentation package
          docs = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
            cargoDocExtraArgs = "--all-features --no-deps --document-private-items";
            RUSTDOCFLAGS = "--cfg docsrs -D warnings";
          });
        };

        # Development shell
        devShells.default = pkgs.mkShell {
          inputsFrom = [ commonArgs ];

          nativeBuildInputs = with pkgs; [
            # Core Rust toolchain (always needed)
            rustToolchain      # cargo, rustc, rustfmt, clippy, rust-analyzer
            rust-analyzer      # IDE support

            # Essential cargo tools (daily use)
            cargo-edit         # cargo add/rm/upgrade commands
            cargo-watch        # Auto-run on file changes
            cargo-nextest      # Faster test runner
            cargo-expand       # Macro debugging

            # Quality & security (frequent use)
            cargo-audit        # Security vulnerability checking
            cargo-outdated     # Dependency updates
            cargo-machete      # Unused dependency detection
            cargo-deny         # License and security policy

            # Performance tools (occasional use)
            cargo-tarpaulin    # Code coverage
            cargo-criterion    # Benchmarking

            # Development tools
            git                # Version control
            gh                 # GitHub CLI
            just               # Task runner
          ];

          shellHook = ''
            echo "🦀 Singularity Language Registry Development Shell"
            echo ""
            echo "Quick commands:"
            echo "  just           - Show all available tasks"
            echo "  just verify    - Run all checks (fmt, clippy, test, audit)"
            echo "  just watch     - Auto-run tests on file changes"
            echo ""
            echo "Cargo commands:"
            echo "  cargo nextest run    - Fast test runner"
            echo "  cargo add <crate>    - Add dependency"
            echo "  cargo audit          - Security check"
            echo "  cargo expand         - Debug macros"
            echo ""
            echo "Nix commands:"
            echo "  nix flake check      - Run all CI checks locally"
            echo "  nix build            - Build the package"
            echo ""

            # Setup git hooks if they exist
            if [ -f ./setup-hooks.sh ]; then
              ./setup-hooks.sh 2>/dev/null || true
            fi
          '';

          # Environment variables for development
          RUST_BACKTRACE = 1;
          RUST_LOG = "debug";
        };

        # Checks run by `nix flake check`
        checks = {
          inherit singularity-language-registry;

          # Format check
          fmt = pkgs.runCommand "fmt-check" {} ''
            ${rustToolchain}/bin/cargo fmt --manifest-path ${./Cargo.toml} -- --check
            touch $out
          '';

          # Clippy check with pedantic mode
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery";
          });

          # Audit check
          audit = craneLib.cargoAudit {
            src = ./.;
            inherit (commonArgs) nativeBuildInputs;
          };

          # Documentation check
          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
            cargoDocExtraArgs = "--all-features --no-deps";
          });

          # Test with all features
          test = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--all-features --release";
          });
        };

        # Apps that can be run
        apps = {
          default = flake-utils.lib.mkApp {
            drv = singularity-language-registry;
          };

          # Open documentation in browser
          docs = {
            type = "app";
            program = toString (pkgs.writeShellScript "open-docs" ''
              set -e
              echo "Building documentation..."
              ${rustToolchain}/bin/cargo doc --all-features --no-deps --document-private-items --open
            '');
          };
        };

        # Formatter for nix files
        formatter = pkgs.nixpkgs-fmt;

        # Legacy support (deprecated but kept for backwards compatibility)
        defaultPackage = singularity-language-registry;
        defaultApp = flake-utils.lib.mkApp {
          drv = singularity-language-registry;
        };
      }
    );
}