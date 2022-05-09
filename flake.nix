{
  description = "rust workspace";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?rev=3eb07eeafb52bcbf02ce800f032f18d666a9498d";
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
      rust-version = "1.60.0";
    in flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          rust-overlay.overlay
          (self: super: rec {
            rustc = self.rust-bin.stable.${rust-version}.default.override {
              extensions =
                [ "rust-src" "rustfmt-preview" "llvm-tools-preview" ];
              targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
            };
            cargo = rustc;
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
          git
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
