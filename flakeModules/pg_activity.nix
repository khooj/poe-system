{
  pkgs,
  lib,
  config,
  name,
  ...
}: let
  inherit (lib) types mkPackageOption;
in {
  options = {
    package = mkPackageOption pkgs "pg_activity" {};
    dsn = lib.mkOption {
      type = types.str;
      description = ''
        PostgreSQL DSN
      '';
      example = "postgresql://example@localhost";
    };

    dbName = lib.mkOption {
      type = types.str;
      description = ''
        Database name
      '';
    };

    debugFile = lib.mkOption {
      type = types.str;
      default = "${config.dataDir}/debug.txt";
      description = ''
        Debug file name
      '';
    };
  };

  config = {
    outputs = {
      settings = {
        processes = {
          "${name}" = let
            startScript = pkgs.writeShellApplication {
              name = "start-pg_activity";
              runtimeInputs = [pkgs.coreutils config.package];
              text = ''
                set -euo pipefail
                mkdir -p "${config.dataDir}"
                exec pg_activity -d ${config.dbName} --debug-file ${config.debugFile} ${config.dsn}
              '';
            };
          in {
            command = startScript;
            availability = {
              restart = "on_failure";
              max_restarts = 5;
            };
          };
        };
      };
    };
  };
}
