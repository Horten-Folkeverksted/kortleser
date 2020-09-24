let
  sources = import ./nix/sources.nix;

  rpi_pkgs = import sources.nixpkgs {
    crossSystem = {
      config = "armv6l-unknown-linux-gnueabihf";
    };
  };

  pkgs = import sources.nixpkgs { };

  build = build_pkgs: build_pkgs.rustPlatform.buildRustPackage {
    pname = "kortleser";
    version = "0.3.0";

    src = ./.;

    cargoSha256 = "1yb802n1mmhrr9sk8s5wld31h8wg4xj5g8a1lxf43yzrld32qa23";
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
