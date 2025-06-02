{callPackage}: let
  jaeger = {
    buildGoModule,
    fetchFromGitHub,
    lib,
    stdenvNoCC,
    buildNpmPackage,
    git,
  }: let
    pname = "jaeger";
    version = "1.69.0";
    src = fetchFromGitHub {
      owner = "jaegertracing";
      repo = "jaeger";
      tag = "v${version}";
      hash = "sha256-UuiASOBy88FR/XEAh2UionHJqKsRzf/Z6Fr6C2aOsx4=";
      fetchSubmodules = true;
    };
    ui = buildNpmPackage {
      inherit version;
      pname = "jaeger-query-ui";
      src = "${src}/jaeger-ui";
      npmDepsHash = "sha256-tdqD3ivheKsdFO+Cgksxd0vTCMxPURfLT3F49rkKmw0=";
      buildInputs = [git];
      npmWorkspace = "packages/jaeger-ui";
      postPatch = ''
        patchShebangs scripts
        # find .
        # exit 1
        patchShebangs node_modules
        # cat /build/node_modules/.bin/vite
      '';
      #
      # buildPhase = ''
      #   mkdir ui
      #   find .
      #   cp -r jaeger-ui/packages/jaeger-ui/build/* ui/
      #   find ui -type f | grev -v .gitignore | xargs gzip --no-name
      #   touch -t $$(date -r jaeger-ui/packages/jaeger-ui/build/index.html '+%Y%m%d%H%M.%S') ui/index.html.gz
      # '';
      # installPhase = ''
      #   mv ui/index.html.gz $out
      # '';
    };
  in
    buildGoModule {
      inherit pname version src;
      vendorHash = "sha256-1/TJAr3/0TUz0/ejM3XLdXQENtmEA49skEO/vgrjayE=";
      subPackages = ["cmd/all-in-one" "cmd/jaeger" "cmd/query"];
      preBuild = ''
        mkdir -p cmd/query/app/ui/actual/
        ln -s ${ui} cmd/query/app/ui/actual/index.html.gz
      '';
      meta.mainProgram = "jaeger";
    };

  signoz = {
    buildGoModule,
    fetchFromGitHub,
    glibc,
    stdenv,
    fetchYarnDeps,
    yarnConfigHook,
    yarnBuildHook,
    yarnInstallHook,
    nodejs,
    makeWrapper,
  }: let
    pname = "signoz-ce";
    version = "0.85.3";
    src = fetchFromGitHub {
      owner = "SigNoz";
      repo = "signoz";
      tag = "v${version}";
      hash = "sha256-ntbLUH/JW7T2E5ORf40w8lqOD/y3Zws7sBFM613wVNs=";
    };
    ui = stdenv.mkDerivation (finalAttrs: {
      pname = "${pname}-ui";
      inherit version;
      src = "${src}/frontend";
      yarnOfflineCache = fetchYarnDeps {
        yarnLock = finalAttrs.src + "/yarn.lock";
        hash = "sha256-OMSMG6gzI66NogdiKtGz0taXny2d/TNsmlUIU8RRBcM=";
      };
      preBuild = ''
        yarn --offline postinstall
      '';
      nativeBuildInputs = [
        yarnConfigHook
        yarnBuildHook
        yarnInstallHook
        nodejs
      ];
    });
  in
    buildGoModule {
      inherit pname version src;
      vendorHash = "sha256-8XCffaK/8FNPsO2PPalJ72DQgWHT1YiPeX1YLueECCU=";
      tags = ["timetzdata"];
      ldflags = [
        "-linkmode external"
        "-extldflags '-static'"
        "-s"
        "-w"
        "-X github.com/SigNoz/signoz/pkg/version.variant=community"
        "-X github.com/SigNoz/signoz/pkg/version.version=${version}"
        "-X github.com/SigNoz/signoz/pkg/version.hash=${version}"
        "-X github.com/SigNoz/signoz/pkg/version.time=${version}"
        "-X github.com/SigNoz/signoz/pkg/version.branch=${version}"
      ];
      subPackages = ["pkg/query-service"];
      env.CGO_ENABLED = 1;
      buildInputs = [glibc.static];
      nativeBuildInputs = [makeWrapper];
      postInstall = ''
        makeWrapper $out/bin/query-service $out/bin/signoz \
          --set SIGNOZ_WEB_DIRECTORY ${ui}/lib/node_modules/frontend/build
      '';
      doCheck = false;
      meta.mainProgram = "signoz";
    };
in {
  jaeger = callPackage jaeger {};
  signoz = callPackage signoz {};
}
