self: {
  imports = [
    (import ./module.nix self)
    ./openobserve.nix
  ];
}
