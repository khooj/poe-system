{
  lib,
  wasm-pack,
  naersk,
  wasm-bindgen-cli,
  binaryen, #wasm-opt
}: let
in
  naersk.buildPackage {
    pname = "wasm";
    version = "0.1.0";
    src = ./.;
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
    WASM_PACK_CACHE = ".wasm-pack-cache";
    singleStep = true;
    buildInputs = [binaryen wasm-bindgen-cli];
    buildPhase = ''
      RUST_LOG=info ${lib.getExe wasm-pack} build wasm
    '';
    installPhase = ''
      mkdir -p $out/lib/wasm
      cp -r wasm/pkg/* $out/lib/wasm/
    '';
  }
