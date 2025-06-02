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
  signoz = cfg.package;

  settingsFormat = pkgs.formats.yaml {};
in {
  options = {
    package = mkPackageOption pkgs "signoz" {};

    settings = mkOption {
      type = settingsFormat.type;
      default = {};
      description = ''
        Specify the configuration for signoz in Nix.
      '';
    };

    configFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      description = ''
        Specify a path to a configuration file that signoz should use.
      '';
    };
  };

  config = {
    outputs.settings.processes."${name}" = let
      conf =
        if cfg.configFile == null
        then settingsFormat.generate "config.yaml" cfg.settings
        else cfg.configFile;
    in {
      command = "${getExe signoz} --config ${conf}";
      working_dir = "${cfg.dataDir}";
      availability = {
        restart = "on_failure";
        max_restarts = 5;
      };
    };
  };
}
