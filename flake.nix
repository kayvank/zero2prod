{
  description = "zero2prod";
  inputs = {
    nixpkgs.url = "github:NixOs/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
        cargoTomlContents = builtins.readFile ./Cargo.toml;
        version = (builtins.fromTOML cargoTomlContents).package.version;

        zero2prod = pkgs.rustPlatform.buildRustPackage {
          inherit version;
          name = "zero2prod";
          buildInputs = with pkgs; [ openssl ];
          nativeBuildInputs = with pkgs; [ pkg-config openssl.dev ];

          src = pkgs.lib.cleanSourceWith { src = self; };

          cargoLock.lockFile = ./Cargo.lock;

          GIT_COMMIT_HASH_SHORT = self.shortRev or "unknown";

        };

        packages = {
          zero2prod = zero2prod;
          default = packages.zero2prod;
        };

       overlays.default = final: prev: { zero2prod = packages.zero2prod; };

        gitRev = if (builtins.hasAttr "rev" self) then self.rev else "dirty";
      in {
        inherit packages overlays;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            toolchain
            openssl
            cargo-insta
            pkg-config
            eza
            rust-analyzer-unwrapped
            watchexec
            cowsay
            redis
            postgresql
          ];
          shellHook = ''
            alias ls=eza
            alias find=fd
            echo "rust-dev" | cowsay
            export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
            export CARGO_HOME="$(pwd)/.cargo"
            export PATH="$CARGO_HOME/bin:$PATH"
            export RUST_BACKTRACE=1
            export DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/newsletter'
            export POSTGRES_USER='postgres'
            export POSTGRES_PASSWORD='postgres'
            export POSTGRES_DB='newsletter'
            cargo install --version 0.5.7 sqlx-cli --no-default-features --features postgres
            cargo install cargo-udeps
            cargo install bunyan
          '';
        };
      });
}
