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
  };

  settings = {
    log_location = "services-log.log";

    processes = {
      "stash_receiver" = {
        disabled = true;
        command = "cargo run --release --bin stash_receiver";
        working_dir = "./rust";
      };
      "docker-compose-signoz" = {
        working_dir = "./data/dc-signoz1";
        command = "docker compose up -f docker-compose-signoz.yaml";
      };
    };
  };
}
