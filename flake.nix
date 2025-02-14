{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell.override {
          stdenv = if pkgs.stdenv.isLinux then
            pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv
          else
            pkgs.clangStdenv;
        } {
          packages = with pkgs;
            let
              rust = rust-bin.selectLatestNightlyWith (
                toolchain: toolchain.default.override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                    "rustfmt"
                  ];
                });
            in [
              rust
              mold
            ];
          RUST_LOG="info,galaxy-restore-dump-cli=trace,galaxy-restore-be=trace,galaxy_restore_dump_cli=trace,galaxy_restore_be=trace";
        };
      }
    );
}

