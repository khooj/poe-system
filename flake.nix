{
  description = "poe-system workspace";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    process-compose.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:juspay/services-flake";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk = {
      url = "github:nix-community/naersk";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} (
      {self, ...}: let
        inherit (inputs.services-flake.lib) multiService;
      in {
        imports = [
          inputs.process-compose.flakeModule
          inputs.flake-parts.flakeModules.flakeModules
        ];

        systems = ["x86_64-linux"];

        flake.nixosModules.default = import ./modules self;

        perSystem = {
          pkgs,
          config,
          system,
          ...
        }: {
          _module.args.pkgs = import self.inputs.nixpkgs {
            inherit system;
            config.allowUnfree = true;
            overlays = [
              self.inputs.rust-overlay.overlays.default
            ];
          };

          process-compose."services" = import ./processes.nix {
            inherit inputs multiService;
            inherit (config) packages;
          };

          packages = let
            beamPackages = pkgs.beam.packagesWith pkgs.beam.interpreters.erlang_27;
            rustToolchain = pkgs.rust-bin.stable."1.84.0".minimal.override {
              targets = ["wasm32-unknown-unknown"];
            };
            buildRustCrateForPkgs = pkgs:
              pkgs.buildRustCrate.override {
                defaultCrateOverrides =
                  pkgs.defaultCrateOverrides
                  // {
                    compress-tools = attrs: {
                      buildInputs = [pkgs.libarchive];
                      nativeBuildInputs = [pkgs.pkg-config];
                    };
                  };
                rustc = rustToolchain;
                cargo = rustToolchain;
              };

            rustNix = pkgs.callPackage ./rust/Cargo.nix {
              inherit buildRustCrateForPkgs;
            };
            naersk' = self.inputs.naersk.lib.${system}.override {
              cargo = rustToolchain;
              rustc = rustToolchain;
            };
            poe-system = pkgs.callPackage ./elixir {
              inherit beamPackages;
              rust-elixir = config.packages.rust-elixir;
              rust-wasm = config.packages.rust-wasm;
            };
            additionalPackages = import ./packages.nix {
              inherit (pkgs) callPackage;
            };
          in
            {
              default = config.packages.poe-system;
              rust-elixir = rustNix.workspaceMembers.elixir.build;
              rust-wasm = pkgs.callPackage ./rust/wasm-pkg.nix {
                naersk = naersk';
              };
            }
            // poe-system // additionalPackages;

          devShells.default = let
            bunNode = pkgs.writeShellApplication {
              name = "node";
              runtimeInputs = [pkgs.bun];
              text = ''
                bun "$@"
              '';
            };
          in
            pkgs.mkShell {
              inputsFrom = [
                config.process-compose."services".services.outputs.devShell
              ];

              buildInputs = with pkgs; [
                (rust-bin.stable."1.84.0".default.override {
                  extensions = [
                    "rust-src"
                    "llvm-tools-preview"
                    "rust-analysis"
                  ];
                  targets = ["wasm32-unknown-unknown"];
                })
                # (rust-bin.stable."1.84.0".default.override {
                #   extensions = [
                #     "rust-src"
                #     "llvm-tools-preview"
                #     "rust-analysis"
                #   ];
                #   targets = ["wasm32-unknown-unknown"];
                # })
                sqlite
                openssl
                cmake
                zlib
                gnumake
                (python3.withPackages (ps: with ps; [requests]))
                nixos-shell
                crate2nix
                sqlx-cli
                elixir_1_18

                wget
                dbus
                openssl_3
                glib
                gtk3
                libsoup_2_4
                webkitgtk_6_0
                librsvg
                hashrat
                libarchive
                lz4
                libuv
                cargo-flamegraph
                inotify-tools
                bun
                # bunNode
                nodejs
                cargo-generate
                wasm-pack
                cargo-modules
                graphviz
                playwright-driver.browsers
                playwright
                protobuf
                node2nix
                websocat
                npm-lockfile-fix
                pg_activity
              ];
              nativeBuildInputs = with pkgs; [
                pkg-config
                nixpkgs-fmt
              ];

              shellHook = ''
                export PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}
                export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true
              '';
            };
        };
      }
    );
}
