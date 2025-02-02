{
  description = "rust workspace";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    process-compose.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:juspay/services-flake";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } (
      { self, ... }:
      {
        imports = [
          inputs.process-compose.flakeModule
        ];

        systems = [ "x86_64-linux" ];

        perSystem =
          {
            pkgs,
            config,
            system,
            ...
          }:
          {
            _module.args.pkgs = import self.inputs.nixpkgs {
              inherit system;
              config.allowUnfree = true;
              overlays = [ self.inputs.rust-overlay.overlays.default ];
            };

            process-compose."services" = {
              imports = [
                inputs.services-flake.processComposeModules.default
              ];

              services = {
                cassandra."cass1".enable = false;
                redis."r1" = {
                  enable = false;
                  port = 0;
                  unixSocket = "./redis.sock";
                };
                postgres."pg1" = {
                  enable = true;
                };
              };

              settings = {
                log_location = "services-log.log";

                processes."val1" =
                  let
                    unixSocket = "./valkey.sock";
                    dataDir = "./data/val1";
                    valkeyConfig = pkgs.writeText "valkey.conf" ''
                      unixsocket ${unixSocket}
                      unixsocketperm 0600
                    '';

                    startScript = pkgs.writeShellApplication {
                      name = "start-vakley";
                      runtimeInputs = [
                        pkgs.coreutils
                        pkgs.valkey
                      ];
                      text = ''
                        set -euo pipefail
                        export VALKEYDATA=${dataDir}
                        if [[ ! -d "$VALKEYDATA" ]]; then
                          mkdir -p "$VALKEYDATA"
                        fi

                        exec valkey-server ${valkeyConfig} --dir "$VALKEYDATA"
                      '';
                    };
                    transformedSocketPath = "${dataDir}/${unixSocket}";
                  in
                  {
                    command = startScript;
                    readiness_probe = {
                      exec.command = "${pkgs.valkey}/bin/valkey-cli -s ${transformedSocketPath} 0 ping";
                      initial_delay_seconds = 2;
                      period_seconds = 10;
                      timeout_seconds = 4;
                      success_threshold = 1;
                      failure_threshold = 5;
                    };
                    availability = {
                      restart = "on_failure";
                      max_restarts = 5;
                    };
                  };
              };
            };

            devShells.default = pkgs.mkShell {
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
                  targets = [ "wasm32-unknown-unknown" ];
                })
                trunk

                sqlite
                openssl
                cmake
                zlib
                gnumake
                (python3.withPackages(ps: with ps; [ requests ]))
                nixos-shell
                crate2nix
                nodejs
                sqlx-cli

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
                cassandra-cpp-driver
                libuv
                cargo-flamegraph
              ];
              nativeBuildInputs = with pkgs; [
                pkg-config
                nixpkgs-fmt
              ];

            };
          };
      }
    );
}
