{ lib, pkgs, config, ... }:

let
  ubootEnabled = true;

  gpu-overlay = "vc4-kms-v3d";

in {

  config = {

  # Ignore cec power key
  services.logind.extraConfig = ''
    HandlePowerKey = ignore
  '';

  hardware = {
    # hardware.deviceTree overlaps with raspberry pi config.txt, but
    # only hardware.deviceTree works with U-Boot
    deviceTree = {
      base = "${pkgs.device-tree_rpi}/broadcom";
      overlays = [
        "${pkgs.raspberrypifw}/share/raspberrypi/boot/overlays/${gpu-overlay}.dtbo"
      ];
    };
   
    firmware = [
      pkgs.wireless-regdb
      pkgs.raspberrypiWirelessFirmware
    ];
  };

  # raspberrypi-tools is kind of big but does have some helpful
  # debugging tools when things go wrong, especially with graphics.
  # environment.systemPackages = [ pkgs.raspberrypi-tools ];

  boot = {
    tmpOnTmpfs = true;
    kernelPackages = pkgs.linuxPackages_rpi0;
    kernelParams = [
      # appparently this avoids some common bug in Raspberry Pi.
      "dwc_otg.lpm_enable=0"

      "plymouth.ignore-serial-consoles"
    ]

      ++ lib.optionals ubootEnabled [
        # avoids https://github.com/raspberrypi/linux/issues/3331
        "initcall_blacklist=bcm2708_fb_init"

        # avoids https://github.com/raspberrypi/firmware/issues/1247
        "cma=256M"
      ];
    initrd.kernelModules = [ "vc4" "bcm2835_dma" "i2c_bcm2835" "bcm2835_rng" ];
  };

  nixpkgs.overlays = [(self: super: lib.optionalAttrs (super.stdenv.hostPlatform != super.stdenv.buildPlatform) {
    # Restrict drivers built by mesa to just the ones we need This
    # reduces the install size a bit.
    mesa = (super.mesa.override {
      vulkanDrivers = [];
      driDrivers = [];
      galliumDrivers = ["vc4" "swrast"];
      enableRadv = false;
      withValgrind = false;
      enableOSMesa = false;
      enableGalliumNine = false;
    }).overrideAttrs (o: {
      mesonFlags = (o.mesonFlags or []) ++ ["-Dglx=disabled"];
    });
  })];

  nixpkgs.crossSystem = {
    config = "armv6l-unknown-linux-gnueabihf";
  };
  boot.loader.grub.enable = false;
  boot.loader.raspberryPi = {
    enable = true;
    version = 0;
  
    uboot.enable = ubootEnabled;

    firmwareConfig = ''
      dtoverlay=${gpu-overlay}
    '' + pkgs.stdenv.lib.optionalString pkgs.stdenv.hostPlatform.isAarch64 ''
      arm_64bit=1
    '' + pkgs.stdenv.lib.optionalString (config.nixiosk.raspberryPi.firmwareConfig != null) config.nixiosk.raspberryPi.firmwareConfig;
  };

  fileSystems = lib.mkForce (if ubootEnabled then {
    "/boot/firmware" = {
      device = "/dev/disk/by-label/FIRMWARE";
      fsType = "vfat";
      options = [ "nofail" "noauto" ];
    };
    "/" = {
      device = "/dev/disk/by-label/NIXOS_SD";
      fsType = "ext4";
      autoResize = true;
    };
  } else {
    "/boot" = {
      device = "/dev/disk/by-label/FIRMWARE";
      fsType = "vfat";
    };
    "/" = {
      device = "/dev/disk/by-label/NIXOS_SD";
      fsType = "ext4";
      autoResize = true;
    };
  });

  networking.wireless.enable = true;

  };

}
