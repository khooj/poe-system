{
  description = "rust workspace";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crate2nix, ... }:
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
        crate2nix-pkgs = import crate2nix { inherit pkgs; };
        lib = pkgs.lib;
        project = import ./Cargo.nix { inherit pkgs; };

        buildInputs = with pkgs; [
          sccache
          sqlite
          postgresql
          mysql
          openssl
          pkg-config
          nasm
          cmake
          zlib
          gnumake
          fswatch
          python3
          jq
          crate2nix-pkgs
          nixos-shell
        ];
        nativeBuildInputs = with pkgs; [ rustc cargo pkgconfig nixpkgs-fmt ];

      in rec {
        packages.${myapp} = project.rootCrate.build;
        defaultPackage = packages.${myapp};
        overlay = final: prev: { poe-system = packages.${myapp}; };
        nixosModule = import ./module.nix;

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
