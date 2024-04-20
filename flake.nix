{
  description = "rust workspace";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    let
      myapp = "poe-system";
      rust-version = "1.77.2";
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        lib = pkgs.lib;

        buildInputs = with pkgs; [
          (rust-bin.stable.${rust-version}.default.override {
              extensions =
                [ "rust-src" "llvm-tools-preview" "rust-analysis" ];
              targets = [ "wasm32-unknown-unknown" ];
          })
          trunk

          sqlite
          postgresql
          mysql
          openssl
          cmake
          zlib
          gnumake
          python3
          jq
          nixos-shell
          git
          crate2nix
          vscodium
          nodejs
          curl

          wget
          dbus
          openssl_3
          glib
          gtk3
          libsoup
          webkitgtk
          librsvg
          hashrat
        ];
        nativeBuildInputs = with pkgs; [ pkg-config nixpkgs-fmt ];
        libs = with pkgs; [
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl_3
          librsvg
          vulkan-loader
          llvmPackages_15.llvm
        ];

      in
      rec {
        devShell = with pkgs;
          mkShell {
            name = "rust";
            buildInputs = [ ] ++ buildInputs;
            inherit nativeBuildInputs;

            shellHook = ''
              export PATH=$PATH:$HOME/.cargo/bin:$PWD/app/node_modules/.bin
            '';
          };
      });
}
