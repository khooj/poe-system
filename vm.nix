{ pkgs, ... }:
{
    imports = [ ./module.nix ];

    nix.nixPath = [
        "nixpkgs=${pkgs.path}"
    ];

    nixpkgs.overlays = [
        (self: super: {
            poe-system = (import ./default.nix).default;
        })
    ];

    services.poe-system = {
        enable = true;
    };
}