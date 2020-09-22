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

    cargoSha256 = "1a69q8sjvmhiwzrs6rsizaalnxshcysqlwd4ggwqd2s2spg83g8s";
  };
in

{
  rpiBuild = build rpi_pkgs;
  nativeBuild = build pkgs;

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
    ];
  }).config.system.build.sdImage;
}
