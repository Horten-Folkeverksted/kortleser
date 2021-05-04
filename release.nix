{ nixpkgs ? <nixpkgs> }: #moz_overlay ? <moz_overlay> }:
with (import ./default.nix { nixpkgs = nixpkgs; moz_overlay = null;  });
{
  inherit kortleser kortleserRpi image;
}
