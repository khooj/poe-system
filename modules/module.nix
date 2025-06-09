self: {
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib) types mkOption mkEnableOption mkIf;
  inherit (pkgs.stdenv.hostPlatform) system;
  defaultPkg = self.packages.${system}.default;
  cfg = config.services.poe-system;
  pkg = cfg.package;
in {
  options.services.poe-system = {
    enable = mkEnableOption "enable poe-system service";
    package = mkOption {
      type = types.package;
      default = defaultPkg;
    };
    secretsEnvFile = mkOption {
      type = types.path;
      description = "path to environment variables file with secrets";
    };
    port = mkOption {
      type = types.int;
      description = "listen port";
      default = 4000;
    };
    host = mkOption {
      type = types.str;
      description = "host on which phoenix works (not ip)";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.poe-system = {
      wantedBy = ["multi-user.target"];
      requires = ["network-online.target"];
      after = ["network-online.target"];
      preStart = ''
        ${pkg}/bin/migrate
      '';
      environment = {
        PHX_SERVER = "1";
        PHX_HOST = cfg.host;
        PORT = builtins.toString cfg.port;
        RELEASE_COOKIE = "none";
      };
      serviceConfig = {
        EnvironmentFile = cfg.secretsEnvFile;
        ExecStart = "${pkg}/bin/poe_system start";
        Restart = "on-failure";
        RestartSec = "10";
      };
    };
  };
}
