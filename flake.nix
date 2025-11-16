{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
            "rust-analyzer"
            "llvm-tools"
          ];
          targets = [
            "aarch64-unknown-none-softfloat"
          ];
        };
        naersk' = pkgs.callPackage naersk {
          cargo = rust;
          rustc = rust;
        };
      in
      {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          cargoBuildOptions = l: l ++ [ "--target=aarch64-unknown-none-softfloat" ];
        };
        devShell = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.cargo-binutils
            pkgs.gdb
            pkgs.qemu
          ];
        };
      }
    );
}
