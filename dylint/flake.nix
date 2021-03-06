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
      myapp = "dylint";
    in flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          rust-overlay.overlay
          (self: super: rec {
            rustc = self.rust-bin.fromRustupToolchainFile ./rust-toolchain;
            cargo = rustc;
          })
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        crate2nix-pkgs = import crate2nix { inherit pkgs; };
        lib = pkgs.lib;
        project = import ./Cargo.nix { inherit pkgs; };

        buildInputs = with pkgs; [
          pkg-config
          openssl
          cmake
          gnumake
          jq
          sqlite
          crate2nix-pkgs
          nixos-shell
        ];
        nativeBuildInputs = with pkgs; [ rustc cargo pkgconfig nixpkgs-fmt ];

      in rec {
        packages.${myapp} = project.rootCrate.build;
        defaultPackage = packages.${myapp};
        overlay = final: prev: {
          "${myapp}" = packages.${myapp};
        };

        devShell = with pkgs;
          mkShell {
            name = "rust";
            buildInputs = [ ] ++ buildInputs;
            inherit nativeBuildInputs;

            shellHook = ''
              export RUSTUP_TOOLCHAIN="nightly-2021-04-22"
              export PATH=$PATH:$HOME/.cargo/bin
            '';
          };
      });
}
