let
  sources = import ./nix/sources.nix;

  pkgs = import sources.nixpkgs { };
  stuff = import ./default.nix;
in
pkgs.stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs = [stuff.shell_pkgs];
}
