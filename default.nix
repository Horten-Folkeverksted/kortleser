{
  nixpkgs ? ((import ./nix/sources.nix).nixpkgs),
}:

let
  rpi_pkgs = import nixpkgs {
    crossSystem = {
      config = "armv6l-unknown-linux-gnueabihf";
    };
  };

  pkgs = import nixpkgs { };

  build = build_pkgs: build_pkgs.rustPlatform.buildRustPackage {
    pname = "kortleser";
    version = "0.3.0";

    src = ./kortleser;

    cargoSha256 = "0apj8xzchak2vn3ghqwi75y79g33r924ksb8fb1786q2zmv1dmk2";
  };
in
{
  kortleserRpi = build rpi_pkgs;
  kortleser = build pkgs;

  image = {system ? null}: (import (pkgs.path + /nixos/lib/eval-config.nix) {
    modules = [
      boot/rpi-uboot.nix
      boot/configuration.nix
      ({lib, ...}: {
        nixpkgs.localSystem = lib.mkForce {
          system = if system != null then system
                   else if builtins.currentSystem == "x86_64-darwin" then "x86_64-linux"
                   else builtins.currentSystem;
        };
      })
      ({lib, ...}: {
        nixpkgs.overlays = [
          (self: super: {
            kortleser = build super;
          })
        ];
      })
    ];
  }).config.system.build.sdImage;
}
