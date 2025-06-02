{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      perSystem = {lib, pkgs, ...}: {
        packages.default = pkgs.buildNpmPackage {
          pname = "joshprk-me";
          version = "0.1.0";
          src = ./.;

          npmDepsHash = "sha256-A2UtALa0THnk0Qh3CIAzNIGf4evcCwRtqSDp2vrY/Kg=";
        };
      };
    };
}
