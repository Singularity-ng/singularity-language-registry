{
  description = "Singularity Language Registry - Production-ready language detection for code analysis";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
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
        };

        # Development shell
        devShells.default = pkgs.mkShell {
          inputsFrom = [ commonArgs ];

          nativeBuildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            cargo-edit
            cargo-watch
            cargo-audit
            cargo-outdated
            cargo-tarpaulin
            cargo-nextest
            cargo-machete
            cargo-deny
            cargo-expand
            cargo-criterion

            # Development tools
            git
            gh
            just
            watchexec
            hyperfine
            tokei

            # Code quality tools
            typos
            taplo
            yamlfmt
          ];

          shellHook = ''
            echo "🦀 Singularity Language Registry Development Shell"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run tests"
            echo "  cargo clippy         - Run clippy lints"
            echo "  cargo fmt            - Format code"
            echo "  cargo audit          - Check for vulnerabilities"
            echo "  cargo outdated       - Check for outdated dependencies"
            echo "  cargo tarpaulin      - Generate code coverage"
            echo "  cargo nextest run    - Run tests with nextest"
            echo "  cargo watch -x test  - Watch and test"
            echo ""
            echo "Nix commands:"
            echo "  nix build            - Build the package"
            echo "  nix flake check      - Run all checks"
            echo "  nix develop          - Enter dev shell"
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
        apps.default = flake-utils.lib.mkApp {
          drv = singularity-language-registry;
        };

        # Formatter for nix files
        formatter = pkgs.nixpkgs-fmt;

        # Legacy support
        defaultPackage = packages.default;
        defaultApp = apps.default;
        devShell = devShells.default;
      }
    );
}