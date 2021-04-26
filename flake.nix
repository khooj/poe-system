{
  description = "rust workspace";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    moz_overlay.url = "github:mozilla/nixpkgs-mozilla/master";
    moz_overlay.flake = false;
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, moz_overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        mozilla_overlay = import moz_overlay;
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ mozilla_overlay ];
        };
        lib = pkgs.lib;
        ruststable = pkgs.latest.rustChannels.stable.rust.override {
          extensions =
            [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ];
        };
      in {
        devShell = with pkgs;
          mkShell {
            name = "rust";
            buildInputs = [
              sqlite
              postgresql
              mysql
              ruststable
              openssl
              pkg-config
              nasm
              rustup
              cmake
              zlib
              gnumake
              fswatch
              python3
              jq
            ];

            shellHook = ''
              export PATH=$PATH:$HOME/.cargo/bin
            '';
          };
      });
}
