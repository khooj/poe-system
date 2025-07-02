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
    sha256 = "sha256-Dqwtd98GBWqp2IQ961hFoxzdYuuLvFYlGdcYWi9s/Fo=";
  };
  assets = buildNpmPackage {
    pname = "poe-system-assets";
    inherit version;
    src = "${src}/assets";
    npmDepsHash = "sha256-82TdvII3l6se1dAnFIwIT2a9+banuWYHiF23nvVQMVs=";
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
      mkdir -p _build/prod/lib/poe_system/priv/
      ln -s ${assets} _build/prod/lib/poe_system/priv/static
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
