{ pkgs ? import <nixpkgs> { }, ruststable }:
with pkgs; mkShell {
  name = "rust";
  buildInputs = [
postgresql
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
  ];

shellHook = ''
export PATH=$PATH:$HOME/.cargo/bin
'';
}
