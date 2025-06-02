{
  pkgs,
  lib,
  config,
  name,
  ...
}: let
  inherit (lib) types mkPackageOption;
in {
  options = {
    package = mkPackageOption pkgs "valkey" {};
    bind = lib.mkOption {
      type = types.nullOr types.str;
      default = "127.0.0.1";
      description = ''
        The IP interface to bind to.
        `null` means "all interfaces".
      '';
      example = "127.0.0.1";
    };

    port = lib.mkOption {
      type = types.port;
      default = 6379;
      description = ''
        The TCP port to accept connections.

        If port is set to `0`, redis will not listen on a TCP socket.
      '';
      apply = v:
        lib.warnIf ((config.unixSocket != null) && (v != 0)) ''
          `${name}` is listening on both the TCP port and Unix socket, set `port = 0;` to listen on only the Unix socket
        ''
        v;
    };

    unixSocket = lib.mkOption {
      type = types.nullOr types.str;
      default = null;
      description = ''
        The path to the socket to bind to.

        If a relative path is used, it will be relative to `dataDir`.
      '';
    };

    unixSocketPerm = lib.mkOption {
      type = types.int;
      default = 660;
      description = "Change permissions for the socket";
      example = 600;
    };

    extraConfig = lib.mkOption {
      type = types.lines;
      default = "";
      description = "Additional text to be appended to `valkey.conf`.";
    };
  };

  config = {
    outputs = {
      settings = {
        processes = {
          "${name}" = let
            valkeyConfig = pkgs.writeText "valkey.conf" ''
              port ${toString config.port}
              ${lib.optionalString (config.bind != null) "bind ${config.bind}"}
              ${lib.optionalString (config.unixSocket != null) "unixsocket ${config.unixSocket}"}
              ${lib.optionalString (config.unixSocket != null) "unixsocketperm ${builtins.toString config.unixSocketPerm}"}
              ${config.extraConfig}
            '';

            startScript = pkgs.writeShellApplication {
              name = "start-valkey";
              runtimeInputs = [pkgs.coreutils config.package];
              text = ''
                set -euo pipefail

                export VALKEYDATA=${config.dataDir}

                if [[ ! -d "$VALKEYDATA" ]]; then
                  mkdir -p "$VALKEYDATA"
                fi

                exec valkey-server ${valkeyConfig} --dir "$VALKEYDATA"
              '';
            };
          in {
            command = startScript;

            readiness_probe = let
              # Transform `unixSocket` by prefixing `config.dataDir` if a relative path is used
              transformedSocketPath =
                if (config.unixSocket != null && (! lib.hasPrefix "/" config.unixSocket))
                then "${config.dataDir}/${config.unixSocket}"
                else config.unixSocket;
            in {
              exec.command =
                if (transformedSocketPath != null && config.port == 0)
                then "${config.package}/bin/valkey-cli -s ${transformedSocketPath} ${toString config.port} ping"
                else "${config.package}/bin/valkey-cli -p ${toString config.port} ping";
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
        };
      };
    };
  };
}
