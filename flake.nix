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
            gflags
            ghostscript_headless
            gmp
            patch
            pkg-config
            readline
            ruby
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
