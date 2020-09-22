let
  sources = import ./nix/sources.nix;
  moz_overlay = import sources.nixpkgs-mozilla;
  nixpkgs = import sources.nixpkgs { overlays = [ moz_overlay ]; };
  rustNightlyChannel = (nixpkgs.rustChannelOf { date = "2020-09-09"; channel = "nightly"; }).rust.override {
    extensions = [
			"rust-src"
			"rls-preview"
			"clippy-preview"
			"rustfmt-preview"
		];
  };
  rustStableChannel = nixpkgs.latest.rustChannels.stable.rust.override {
    extensions = [
      "rust-src"
      "rls-preview"
      "clippy-preview"
      "rustfmt-preview"
    ];
  };
in
with nixpkgs;
stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs = [
    niv
    rustStableChannel
  ];
}
