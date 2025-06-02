multiService: {
  imports = builtins.map multiService [
    ./valkey.nix
    ./victoriametrics.nix
    ./otel-collector.nix
    ./jaeger.nix
    ./signoz.nix
  ];
}
