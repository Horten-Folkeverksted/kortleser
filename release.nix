{ nixpkgs ? <nixpkgs> }: #moz_overlay ? <moz_overlay> }:
with (import ./default.nix {inherit nixpkgs; });
{
  inherit kortleser kortleserRpi image;
}
