{
  beamPackages,
  lib,
  rust-elixir,
  rust-wasm,
  buildNpmPackage,
  makeWrapper,
  bun,
}: let
  inherit (beamPackages) mixRelease fetchMixDeps;
  version = "0.1.0";
  src = lib.cleanSource ./.;
  mixFodDeps = fetchMixDeps {
    inherit version src;
    pname = "poe-system-deps";
    sha256 = "sha256-cI8+g591kr2IDF0GSFxTC/ukwk4Of4c5TZqLNgQrGD8=";
  };
  assets = buildNpmPackage {
    pname = "poe-system-assets";
    inherit version;
    src = "${src}/assets";
    npmDepsHash = "sha256-K2MAU0ONhqYJeapSwI7vjezsZMMoffsBt0iJgF2z31w=";
    preBuild = ''
      rm ./node_modules/wasm
      mkdir -p ./node_modules/wasm
      cp -r ${rust-wasm}/lib/wasm/* ./node_modules/wasm/
    '';
    installPhase = ''
      runHook preInstall
      mkdir -p $out
      cp -r result/* $out/
      runHook postInstall
    '';
  };
in {
  poe-system = mixRelease {
    inherit version src mixFodDeps;
    pname = "poe-system";
    nativeBuildInputs = [makeWrapper];
    postBuild = ''
      mkdir -p _build/prod/lib/poe_system/priv/static/
      ln -s ${assets} _build/prod/lib/poe_system/priv/static/assets
      mkdir -p _build/prod/lib/poe_system/priv/native
      ln -s ${rust-elixir.lib}/lib/libelixir.so _build/prod/lib/poe_system/priv/native/libelixir.so
    '';
    postInstall = ''
      wrapProgram $out/bin/poe_system --prefix PATH : ${lib.makeBinPath [bun]}
    '';
    meta.mainProgram = "poe_system";
  };
  poe-system-assets = assets;
}
