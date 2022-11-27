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
      rust-version = "1.62.0";
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays2 = [
          rust-overlay.overlays.default
          (self: super: rec {
            rustc = self.rust-bin.stable.${rust-version}.default.override {
              extensions =
                [ "rust-src" "rustfmt" "llvm-tools-preview" "rust-analysis" "clippy" "rust-std" ];
              targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
            };
            cargo = rustc;
          })
        ];
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        lib = pkgs.lib;

        buildInputs = with pkgs; with rust-bin.stable.${rust-version}; [
          rustc
          cargo
          rustfmt
          clippy
          rust-std
          rust-analyzer

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
          git
          crate2nix
          vscodium
        ];
        nativeBuildInputs = with pkgs; [ rustc cargo pkgconfig nixpkgs-fmt ];

      in
      rec {
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
