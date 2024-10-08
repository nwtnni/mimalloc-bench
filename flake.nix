# https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
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
            numactl
            patch
            pkg-config
            readline
            rr
            ruby
            snappy
            tbb
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
