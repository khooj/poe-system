{ config, pkgs, lib, ... }:
with lib;
let
  cfg = config.services.poe-system;
  pkg = pkgs.poe-system;
in {
  options.services.poe-system = {
    database_url = lib.mkOption {
      type = types.str;
      default = "/var/lib/poe-system/main.db";
      description = "database connection url";
    };
    start_change_id = lib.mkOption {
      type = types.str;
      default = "";
      description =
        "next_change_id variable to start with if db does not already have one";
    };
    enable = lib.mkOption {
      type = types.bool;
      default = true;
      description = "enable service";
    };
  };

  config = {
    systemd.services.poe-system = lib.mkIf cfg.enable {
      wantedBy = [ "multi-user.target" ];
      requires = [ ];
      after = [ ];
      preStart = ''
        d=$(dirname ${cfg.start_change_id})
        mkdir -p $d
      '';
      environment = {
          DATABASE_URL = cfg.database_url;
          START_CHANGE_ID = cfg.start_change_id;
      };
      serviceConfig = {
        ExecStart = "${pkg}/bin/main";
        Restart = "on-failure";
        RestartSec = "10";
        User = "poe-system";
      };
    };

    users.users.poe-system = { isNormalUser = false; };
  };
}
