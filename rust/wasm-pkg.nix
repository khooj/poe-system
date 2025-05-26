{
  lib,
  stdenvNoCC,
}: let
in
  stdenvNoCC.mkDerivation (finalAttrs: {
    pname = "wasm";
    version = "0.1.0";

    src = ./wasm/pkg;

    installPhase = ''
      mkdir -p $out/pkg
      cp -r $src/* $out/pkg/
    '';
  })
