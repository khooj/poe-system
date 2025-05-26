{
  beamPackages,
  lib,
  rust-elixir,
  rust-wasm,
  buildNpmPackage,
  fetchNpmDeps,
}: let
  inherit (beamPackages) mixRelease fetchMixDeps;
  version = "0.1.0";
  src = lib.cleanSource ./elixir;
  mixFodDeps = fetchMixDeps {
    inherit version src;
    pname = "poe-system-deps";
    sha256 = "sha256-quTdv0ZNE7cpnED54BoTvZdfSSa9IoZhXAAi+DZ2ZxM=";
  };
  assets = buildNpmPackage {
    pname = "poe-system-assets";
    inherit version;
    src = "${src}/assets";
    npmDeps = fetchNpmDeps {
      hash = "sha256-XBTIa4oNiWQhGXRliv4KPetVQoVYEcK3ZkxQZsSnMuw=";
      src = "${src}/assets";
    };
    postBuild = ''
      mkdir -p $out/lib/rust/wasm/pkg
      cp -r ${rust-wasm}/pkg/* $out/lib/rust/wasm/pkg/
    '';
  };
in
  mixRelease {
    inherit version src mixFodDeps;
    pname = "poe-system";

    postBuild = ''
      find .
      mkdir -p _build/prod/lib/poe_system/priv/static/assets
      cp -r ${assets}/ _build/prod/lib/poe_system/priv/static/assets/
      mkdir -p _build/prod/lib/poe_system/priv/native
      cp ${rust-elixir.lib}/lib/libelixir.so _build/prod/lib/poe_system/priv/native/
    '';
    meta.mainProgram = "poe-system";
  }
