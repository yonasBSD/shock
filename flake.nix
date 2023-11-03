{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs@{ self, nixpkgs, ... }:
    let
      supportedSystems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      myPkgsFor = pkgs: import ./nix/pkgs {
        inherit pkgs inputs;
        craneLib = inputs.crane.mkLib pkgs;
      };
    in
    {
      overlays.default = final: prev: myPkgsFor final;

      nixosModules = rec {
        default = shock;
        shock = import ./nix/modules/shock.nix;
      };
    } // inputs.flake-utils.lib.eachSystem supportedSystems (system:
      let
        mkPkgs = system: import nixpkgs {
          inherit system;
        };

        pkgs = mkPkgs system;
        myPkgs = myPkgsFor pkgs;
        checks = myPkgs // (import ./nix/checks.nix { inherit pkgs myPkgs; });
      in
      {
        inherit checks;

        formatter = pkgs.nixpkgs-fmt;

        packages = myPkgs // { default = myPkgs.shock; };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues checks;
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            clippy
          ];
        };
      });
}
