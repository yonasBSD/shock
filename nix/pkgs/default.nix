{ pkgs, inputs, craneLib }:

{
  shock = pkgs.callPackage ./shock.nix {
    inherit craneLib;
  };
}
