{ pkgs, ... }:
{
    imports = [ ./module.nix ];

    nix.nixPath = [
        "nixpkgs=${pkgs.path}"
    ];

    nixpkgs.overlays = [
        (self: super: {

        })
    ]

    services.poe-system = {
        enable = true;
    };
}