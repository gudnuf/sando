{
  description = "Sando - A Rust web application for managing database connection strings";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl.dev
          cmake
          gcc
          gnumake
          perl
          python3
          protobuf
        ];

        buildInputs = with pkgs; [
          sqlite
          openssl
          zlib
          libz
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        # Environment variables for development
        devEnvVars = {
          DATABASE_URL = "sqlite://connections.db";
          RUST_LOG = "info";
          RUST_BACKTRACE = "1";
        };

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          
          shellHook = ''
            echo "Welcome to Sando development environment!"
            echo "Database URL: $DATABASE_URL"
            echo ""
            echo "Available commands:"
            echo "  cargo run       - Run the application"
            echo "  cargo build     - Build the application"
            echo "  cargo test      - Run tests"
            echo "  cargo sqlx      - SQLx CLI commands"
            echo ""
            
            # Create database file if it doesn't exist
            if [ ! -f connections.db ]; then
              echo "Creating database file..."
              touch connections.db
            fi
            
            # Run migrations if sqlx-cli is available
            if command -v sqlx &> /dev/null; then
              echo "Running database migrations..."
              sqlx migrate run || echo "Migration failed or already applied"
            fi
          '';
          
          # Set environment variables
          DATABASE_URL = devEnvVars.DATABASE_URL;
          RUST_LOG = devEnvVars.RUST_LOG;
          RUST_BACKTRACE = devEnvVars.RUST_BACKTRACE;
          
          # Additional tools for development
          packages = with pkgs; [
            sqlx-cli
            cargo-watch
            cargo-edit
            rust-analyzer
          ];
        };

        # Package for building the application
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "sando";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          inherit nativeBuildInputs buildInputs;
          
          # Set build-time environment variables
          DATABASE_URL = devEnvVars.DATABASE_URL;
          
          # Copy static files to output
          postInstall = ''
            mkdir -p $out/share/sando
            cp -r static $out/share/sando/
            cp -r migrations $out/share/sando/
          '';
        };

        # App for running the application
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
          exePath = "/bin/sando";
        };
        
        # Alternative run script with environment setup
        apps.run-with-setup = flake-utils.lib.mkApp {
          drv = pkgs.writeShellScriptBin "sando-run" ''
            export DATABASE_URL="${devEnvVars.DATABASE_URL}"
            export RUST_LOG="${devEnvVars.RUST_LOG}"
            
            # Ensure database exists
            if [ ! -f connections.db ]; then
              echo "Creating database file..."
              touch connections.db
            fi
            
            # Run migrations
            if command -v sqlx &> /dev/null; then
              echo "Running database migrations..."
              ${pkgs.sqlx-cli}/bin/sqlx migrate run || echo "Migration failed or already applied"
            fi
            
            # Run the application
            exec ${self.packages.${system}.default}/bin/sando "$@"
          '';
        };
      });
} 