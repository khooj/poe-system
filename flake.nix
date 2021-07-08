{
  description = "rust workspace";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    let
      myapp = "poe-system";
      rust-version = "1.51.0";
    in flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          rust-overlay.overlay
          (self: super: rec {
            rustc = self.rust-bin.stable.${rust-version}.default.override {
              extensions =
                [ "rust-src" "rustfmt-preview" "llvm-tools-preview" ];
            };
            cargo = rustc;
          })
          (self: super: rec {
            rustc-nightly =
              self.rust-bin.nightly."2021-04-22".default.override {
                extensions = [ "rustc-dev" "llvm-tools-preview" ];
              };
          })
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        lib = pkgs.lib;

        buildInputs = with pkgs; [
          sccache
          sqlite
          postgresql
          mysql
          openssl
          pkg-config
          cmake
          zlib
          gnumake
          python3
          jq
          nixos-shell
        ];
        nativeBuildInputs = with pkgs; [ rustc cargo pkgconfig nixpkgs-fmt ];

      in rec {
        devShell = with pkgs;
          mkShell {
            name = "rust";
            buildInputs = [ ] ++ buildInputs;
            inherit nativeBuildInputs;

            shellHook = ''
              export RUSTC_WRAPPER="${pkgs.sccache}/bin/sccache"
              export PATH=$PATH:$HOME/.cargo/bin
            '';
          };
      });
}
