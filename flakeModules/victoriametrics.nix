{
  name,
  pkgs,
  config,
  lib,
  ...
}: let
  inherit (lib) types mkPackageOption mkOption literalExpression;
  settingsFormat = pkgs.formats.yaml {};
  startCLIList =
    [
      "${config.package}/bin/victoria-metrics"
      "-storageDataPath=${config.stateDir}"
      "-httpListenAddr=${config.listenAddress}"
    ]
    ++ lib.optionals (config.retentionPeriod != null) ["-retentionPeriod=${config.retentionPeriod}"]
    ++ config.extraOptions;

  prometheusConfigYml = checkedConfig (
    settingsFormat.generate "prometheusConfig.yaml" config.prometheusConfig
  );

  checkedConfig = file:
    pkgs.runCommand "checked-config" {nativeBuildInputs = [config.package];} ''
      ln -s ${file} $out
      ${lib.escapeShellArgs startCLIList} -promscrape.config=${file} -dryRun
    '';
in {
  options = {
    package = mkPackageOption pkgs "victoriametrics" {};

    listenAddress = mkOption {
      default = "127.0.0.1:8428";
      type = types.str;
      description = ''
        TCP address to listen for incoming http requests.
      '';
    };

    stateDir = mkOption {
      type = types.str;
      default = "./data/${name}";
      description = ''
        Directory below `/var/lib` to store VictoriaMetrics metrics data.
        This directory will be created automatically using systemd's StateDirectory mechanism.
      '';
    };

    retentionPeriod = mkOption {
      type = types.nullOr types.str;
      default = null;
      example = "15d";
      description = ''
        How long to retain samples in storage.
        The minimum retentionPeriod is 24h or 1d. See also -retentionFilter
        The following optional suffixes are supported: s (second), h (hour), d (day), w (week), y (year).
        If suffix isn't set, then the duration is counted in months (default 1)
      '';
    };

    prometheusConfig = lib.mkOption {
      type = lib.types.submodule {freeformType = settingsFormat.type;};
      default = {};
      example = literalExpression ''
        {
          scrape_configs = [
            {
              job_name = "postgres-exporter";
              metrics_path = "/metrics";
              static_configs = [
                {
                  targets = ["1.2.3.4:9187"];
                  labels.type = "database";
                }
              ];
            }
            {
              job_name = "node-exporter";
              metrics_path = "/metrics";
              static_configs = [
                {
                  targets = ["1.2.3.4:9100"];
                  labels.type = "node";
                }
                {
                  targets = ["5.6.7.8:9100"];
                  labels.type = "node";
                }
              ];
            }
          ];
        }
      '';
      description = ''
        Config for prometheus style metrics.
        See the docs: <https://docs.victoriametrics.com/vmagent/#how-to-collect-metrics-in-prometheus-format>
        for more information.
      '';
    };

    extraOptions = mkOption {
      type = types.listOf types.str;
      default = [];
      example = literalExpression ''
        [
          "-httpAuth.username=username"
          "-httpAuth.password=file:///abs/path/to/file"
          "-loggerLevel=WARN"
        ]
      '';
      description = ''
        Extra options to pass to VictoriaMetrics. See the docs:
        <https://docs.victoriametrics.com/single-server-victoriametrics/#list-of-command-line-flags>
        or {command}`victoriametrics -help` for more information.
      '';
    };
  };

  config.outputs.settings.processes."${name}" = let
  in {
    command = pkgs.writeShellApplication {
      name = "start-victoriametrics";
      text = lib.escapeShellArgs (
        startCLIList
        ++ lib.optionals (config.prometheusConfig != {}) ["-promscrape.config=${prometheusConfigYml}"]
      );
    };
    readiness_probe = {
      initial_delay_seconds = 2;
      period_seconds = 10;
      timeout_seconds = 4;
      success_threshold = 1;
      failure_threshold = 5;
    };
    availability = {
      restart = "on_failure";
      max_restarts = 5;
    };
  };
}
