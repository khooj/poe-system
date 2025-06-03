{
  inputs,
  multiService,
  packages,
}: {
  imports = [
    inputs.services-flake.processComposeModules.default
    (import ./flakeModules multiService)
  ];

  services = {
    postgres."pg1" = {
      enable = true;
    };
    pgadmin."pgadm1" = {
      enable = true;
      initialEmail = "example@example.com";
      initialPassword = "12345678";
    };
    valkey."val1" = {
      enable = false;
      unixSocket = "./valkey.sock";
    };
    openobserve."oo1" = {
      enable = true;
      initialEmail = "admin@admin.org";
      initialPassword = "12345678";
    };
    otel-collector."otc1" = {
      enable = true;
      settings = {
        receivers = {
          prometheus.config.scrape_configs = [
            {
              job_name = "otel-collector";
              scrape_interval = "5s";
              static_configs = [{targets = ["localhost:4021"];}];
            }
          ];
        };
        processors.batch = {};
        exporters = {
          "otlphttp/openobserve" = {
            endpoint = "http://localhost:5080/api/default";
            headers = {
              "Authorization" = "Basic YWRtaW5AYWRtaW4ub3JnOlJUeGJKa0lSMEJ0Z2FISWs=";
              "stream-name" = "default";
            };
          };
        };
        service.pipelines = {
          metrics = {
            receivers = ["prometheus"];
            processors = ["batch"];
            exporters = ["otlphttp/openobserve"];
          };
        };
      };
    };
  };

  settings = {
    log_location = "services-log.log";

    processes = {
      "stash_receiver" = {
        disabled = true;
        command = "cargo run --release --bin stash_receiver";
        working_dir = "./rust";
      };
    };
  };
}
