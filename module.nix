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
  };

  config = mkIf cfg.enable {
    systemd.services.poe-system = {
      wantedBy = ["multi-user.target"];
      requires = ["network-online.target"];
      after = ["network-online.target"];
      environment = {
        PHX_SERVER = "1";
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
