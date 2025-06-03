{
  name,
  config,
  pkgs,
  lib,
  ...
}: let
  inherit
    (lib)
    mkEnableOption
    mkPackageOption
    mkIf
    mkOption
    types
    getExe
    ;
  cfg = config;
  openobserve = cfg.package;

  settingsFormat = pkgs.formats.yaml {};
in {
  options = {
    package = mkPackageOption pkgs "openobserve" {};
    initialEmail = mkOption {
      type = types.str;
      description = "initial email for root user";
    };
    initialPassword = mkOption {
      type = types.str;
      description = "initial password for root user";
    };
    extraEnv = mkOption {
      type = types.attrs;
      description = "additional enviroment variables for configuration";
      default = {};
    };
  };

  config = {
    outputs.settings.processes."${name}" = let
      command = pkgs.writeShellApplication {
        name = "start-oo";
        text = ''
          mkdir -p ${cfg.dataDir}
          ${getExe openobserve}
        '';
      };
    in {
      inherit command;
      environment =
        {
          ZO_ROOT_USER_EMAIL = cfg.initialEmail;
          ZO_ROOT_USER_PASSWORD = cfg.initialPassword;
          ZO_DATA_DIR = cfg.dataDir;
        }
        // cfg.extraEnv;
      availability = {
        restart = "on_failure";
        max_restarts = 5;
      };
    };
  };
}
