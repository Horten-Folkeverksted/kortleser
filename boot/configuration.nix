{ lib, pkgs, config, kortleser, ...}: {

  imports = [
    ./raspberrypi.nix
  ];

  ## custom
  time.timeZone = "Europe/Oslo";
  i18n = {
    defaultLocale = "en_US";
    supportedLocales = [ "en_US.UTF-8/UTF-8" ];
  };
  boot.extraModprobeConfig = ''
    options cfg80211 ieee80211_regdom="EU"
  '';
  
  networking.hostName = "kortleser-nix";
  networking.wireless.networks = {
    "folkeverkstedet" = { pskRaw = "8bc9347897ecea03b09d7f320f021cbbddf6aa6d3ba64b4c49e88065ba54c447"; };
  };

  ## configuration
  hardware.opengl.enable = false;
  hardware.bluetooth.enable = false;
  sound.enable = false;
  hardware.pulseaudio.enable = false;
  services.dbus.enable = true;
  services.dbus.socketActivated = true;

  # input
  services.udev.packages = [ pkgs.libinput.out ];

  services.openssh = {
    enable = true;
    permitRootLogin = "without-password";
  };

  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCp8iMOx3eTiG5AmDh2KjKcigf7xdRKn9M7iZQ4RqP0np0UN2NUbu+VAMJmkWFyi3JpxmLuhszU0F1xY+3qM3ARduy1cs89B/bBE85xlOeYhcYVmpcgPR5xduS+TuHTBzFAgp+IU7/lgxdjcJ3PH4K0ruGRcX1xrytmk/vdY8IeSk3GVWDRrRbH6brO4cCCFjX0zJ7G6hBQueTPQoOy3jrUvgpRkzZY4ZCuljXtxbuX5X/2qWAkp8ca0iTQ5FzNA5JUyj+DWeEzjIEz6GrckOdV2LjWpT9+CtOqoPZOUudE1J9mJk4snNlMQjE06It7Kr50bpwoPqnxjo7ZjlHFLezl daniel@DanixLaptop"
  ];

  networking.firewall.allowedTCPPorts = [ 22 3333 ];

  systemd.enableEmergencyMode = false;
  #systemd.services."serial-getty@ttyS0".enable = false;
  #systemd.services."serial-getty@hvc0".enable = false;
  systemd.services."getty@tty1".enable = false;
  systemd.services."autovt@".enable = false;

  services.udisks2.enable = false;
  documentation.enable = false;
  powerManagement.enable = false;
  programs.command-not-found.enable = false;

  services.avahi = {
    enable = true;
    nssmdns = true;
    publish = {
      enable = true;
      userServices = true;
      addresses = true;
      hinfo = true;
      workstation = true;
      domain = true;
    };
  };
  environment.etc."avahi/services/ssh.service" = {
    text = ''
      <?xml version="1.0" standalone='no'?><!--*-nxml-*-->
      <!DOCTYPE service-group SYSTEM "avahi-service.dtd">
      <service-group>
        <name replace-wildcards="yes">%h</name>
        <service>
          <type>_ssh._tcp</type>
          <port>22</port>
        </service>
      </service-group>
    '';
  };

  boot.kernelParams = [ "rd.udev.log_priority=3" "vt.global_cursor_default=0" ];

  networking.dhcpcd.extraConfig = ''
    timeout 0
    noarp
  '';

  security.polkit.extraConfig = ''
    polkit.addRule(function(action, subject) {
      if (action.id == "org.freedesktop.login1.power-off" ||
	        action.id == "org.freedesktop.login1.reboot") {
        return polkit.Result.YES;
      }
    });
  '';

  # kortleser

  environment.noXlibs = true;
  services.xserver.enable = false;
 
#  environment.systemPackages = [ pkgs.kortleser ];

  systemd.services.kortleser = {
    wantedBy = [ "multi-user.target" ]; 
    after = [ "network-online.target" ];

    description = "Start kortleser programmet";

    serviceConfig = {
      ExecStart = ''${pkgs.kortleser}/bin/kortleser --host 10.100.100.1 --port 3333'';
      StandardInput = "tty";
      TTYPath = "/dev/tty1";
    };
  };

  users.users.root.initialPassword = "1234";
}
