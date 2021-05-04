{ nixpkgs ? <nixpkgs>, moz_overlay ? <moz_overlay> }:

import ./default.nix {inherit nixpkgs moz_overlay; }
