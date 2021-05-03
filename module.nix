{ config, pkgs, lib, ... }: with lib; let 
cfg = config.services.poe-system;
pkg = pkgs.poe-system;
in {
    options.services.poe-system = {
        database_url = lib.mkOption {
            type = types.str;
            default = null;
            description = "database connection url";
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
            requires = [];
            after = [];
            serviceConfig = {
                ExecStart = "${pkg}/bin/poe-system";
                Restart = "on-failure";
                RestartSec = "10";
            };
        };
    };
}