# https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      with pkgs; {
        devShells.default = mkShell {
          nativeBuildInputs = [
            autoconf
            bc
            cmake
            dos2unix
            gdb
            gflags
            linuxPackages_latest.perf
            ghostscript_headless
            gmp
            patch
            pkg-config
            readline
            rr
            ruby
            rustToolchain
            snappy
            unzip
            wget
            z3
          ];

          buildInputs = [
            (python3.withPackages (p: with p; [
              numpy
              packaging
              pandas
              plotly
              python-lsp-ruff
              python-lsp-server
            ]))
          ];
        };
      }
    );
}
