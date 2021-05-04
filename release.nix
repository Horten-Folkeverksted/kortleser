{ nixpkgs ? <nixpkgs>, moz_overlay ? <moz_overlay> }:
with (import ./default.nix {inherit nixpkgs moz_overlay; });
{
  inherit kortleser kortleserRpi image;
}
