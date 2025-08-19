{
  beamPackages,
  lib,
  rust-elixir,
  makeWrapper,
  bun,
  tailwindcss_4,
  stdenv,
}: let
  inherit (beamPackages) mixRelease fetchMixDeps;
  version = "0.2.0";
  src = lib.cleanSource ./.;
  mixFodDeps = fetchMixDeps {
    inherit version src;
    pname = "poe-system-deps";
    sha256 = "sha256-mdkamY0pckhrsMmmnzspyFTELxieCwjYQXdRGDx+WdA=";
  };
  # FIXME: use phoenix/phoenix_html/phoenix_live_view deps from elixir project deps
  assets = stdenv.mkDerivation {
    pname = "poe-system-assets";
    inherit version src;
    outputHashMode = "recursive";
    outputHash = "sha256-BDANp4SYj+dxFP2TuBmRVn+u5722tcHMj4RjMbGcJWc=";
    buildInputs = [bun tailwindcss_4];
    buildPhase = ''
      cp -R ${mixFodDeps} deps
      cd assets-daisy
      bun install --cache-dir=bun-cache --frozen-lockfile -p
      bun build --cache-dir=bun-cache --frozen-lockfile --outfile build/app.js index.js
      tailwindcss -i app.css -o build/app.css
      mkdir -p $out/lib
      mv build/* $out/lib/
    '';
  };
in {
  poe-system = mixRelease {
    inherit version src mixFodDeps;
    pname = "poe-system";
    nativeBuildInputs = [makeWrapper];
    postBuild = ''
      mkdir -p _build/prod/lib/poe_system/priv/
      ln -s ${assets}/lib _build/prod/lib/poe_system/priv/static
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
