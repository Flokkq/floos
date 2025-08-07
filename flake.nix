{
  description = "Basic Rust flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell rec {
            buildInputs = [
              (rust-bin.selectLatestNightlyWith (toolchain:
                toolchain.default.override {
                  extensions = ["rust-src" "rust-analyzer" "llvm-tools-preview"];
                  targets = ["thumbv7em-none-eabihf"];
                }))
              gnupg
              git-cliff
              pre-commit

              gcc
              rustc
              cargo
              cargo-watch
              clippy
              rustfmt
              pkg-config
              openssl
              cargo-bootimage
              qemu
              cargo-binutils
              gnumake42
            ];

            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          };
        }
    );
}
