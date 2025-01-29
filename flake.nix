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
                  enable = true;
                  port = 0;
                  unixSocket = "./redis.sock";
                };
                postgres."pg1" = {
                  enable = true;
                };
              };

              settings = {
                log_location = "services-log.log";
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
                python3
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
