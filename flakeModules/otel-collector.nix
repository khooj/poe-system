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
  opentelemetry-collector = cfg.package;

  settingsFormat = pkgs.formats.yaml {};
in {
  options = {
    package = mkPackageOption pkgs "opentelemetry-collector" {};

    settings = mkOption {
      type = settingsFormat.type;
      default = {};
      description = ''
        Specify the configuration for Opentelemetry Collector in Nix.

        See <https://opentelemetry.io/docs/collector/configuration/> for available options.
      '';
    };

    configFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      description = ''
        Specify a path to a configuration file that Opentelemetry Collector should use.
      '';
    };
  };

  config = {
    # assertions = [
    #   {
    #     assertion = (cfg.settings == {}) != (cfg.configFile == null);
    #     message = ''
    #       Please specify a configuration for Opentelemetry Collector with either
    #       'services.opentelemetry-collector.settings' or
    #       'services.opentelemetry-collector.configFile'.
    #     '';
    #   }
    # ];
    outputs.settings.processes."${name}" = let
      conf =
        if cfg.configFile == null
        then settingsFormat.generate "config.yaml" cfg.settings
        else cfg.configFile;
    in {
      command = "${getExe opentelemetry-collector} --config=file:${conf}";
      availability = {
        restart = "on_failure";
        max_restarts = 5;
      };
    };
  };
}
