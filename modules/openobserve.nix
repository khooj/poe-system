{
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib) types mkOption mkEnableOption mkIf mkPackageOption;
  cfg = config.services.openobserve;
  pkg = cfg.package;
in {
  options.services.openobserve = {
    enable = mkEnableOption "enable openobserve service";
    package = mkPackageOption pkgs "openobserve" {};
    secretsEnvFile = mkOption {
      type = types.path;
      description = ''
        path to environment variables file with secrets.
        You should provide `ZO_ROOT_USER_EMAIL` and `ZO_ROOT_USER_PASSWORD` for initial credentials
      '';
    };
    extraEnv = mkOption {
      type = types.attrs;
      description = "additional enviroment variables for configuration";
      default = {};
    };
    dataDir = mkOption {
      type = types.str;
      description = "data dir";
      default = "/var/lib/openobserve";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.openobserve = {
      wantedBy = ["multi-user.target"];
      requires = ["network-online.target"];
      after = ["network-online.target"];
      preStart = ''
        mkdir -p ${cfg.dataDir}
      '';
      environment =
        {
          ZO_DATA_DIR = cfg.dataDir;
        }
        // cfg.extraEnv;
      serviceConfig = {
        EnvironmentFile = cfg.secretsEnvFile;
        ExecStart = "${lib.getExe pkg}";
        Restart = "on-failure";
        RestartSec = "10";
      };
    };
  };
}
