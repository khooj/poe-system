{
  config,
  pkgs,
  lib,
  ...
}:
with lib; let
  cfg = config.services.poe-system;
  pkg = pkgs.poe-system;
in {
  options.services.poe-system = {
    databaseUrl = lib.mkOption {
      type = types.str;
      default = "ecto://khooj@localhost:5432/khooj";
      description = "database connection url";
    };
    secretBaseKey = lib.mkOption {
      type = types.str;
      description = "secret base key for phoenix";
    };
    enable = lib.mkOption {
      type = types.bool;
      default = true;
      description = "enable service";
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.poe-system = {
      wantedBy = ["multi-user.target"];
      requires = ["network-online.target"];
      after = ["network-online.target"];
      environment = {
        DATABASE_URL = cfg.databaseUrl;
        SECRET_BASE_KEY = cfg.secretBaseKey;
      };
      serviceConfig = {
        ExecStart = "${pkg}/bin/poe_system start";
        Restart = "on-failure";
        RestartSec = "10";
      };
    };
  };
}
