{
  description = "rust workspace";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    moz_overlay.url = "github:mozilla/nixpkgs-mozilla/master";
    moz_overlay.flake = false;
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, moz_overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        mozilla_overlay = import moz_overlay;
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ mozilla_overlay ];
        };
        lib = pkgs.lib;
        ruststable = pkgs.latest.rustChannels.stable.rust.override {
          extensions =
            [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ];
        };
      in {
        devShell = import ./shell.nix { inherit pkgs ruststable; };
        packages = {
          gst-image = let
            gst-omx-pkg = import ./gst-omx.nix { inherit pkgs; };
          in pkgs.dockerTools.buildImage {
            name = "gst-image";
            tag = "latest";
            contents = with pkgs; [
              gst_all_1.gst-devtools
              gst_all_1.gst-libav
              gst_all_1.gst-plugins-base
              gst_all_1.gst-plugins-good
              gst_all_1.gst-plugins-ugly
              gst_all_1.gst-rtsp-server
              gst_all_1.gstreamer
              gst_all_1.gstreamermm
              gst-omx-pkg.gst-omx
              gst-omx-pkg.gst-omx-conf
            ];

            config = {
              Cmd = [ "gst-launch-1.0" ];
            };
          };
        };
      });
}
