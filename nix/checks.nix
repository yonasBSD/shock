{ pkgs, myPkgs }:

let
  inherit (pkgs) lib;

  renameChecksForPkg = pname: lib.mapAttrs' (k: lib.nameValuePair "${pname}-${k}");
  renamedChecks = lib.mapAttrsToList (pname: drv: renameChecksForPkg pname (drv.flakeChecks or { })) myPkgs;
  mergedChecks = lib.foldl (a: b: a // b) { } renamedChecks;
in
mergedChecks // {
  nixpkgs-fmt = pkgs.runCommand "nixpkgs-fmt"
    {
      nativeBuildInputs = [ pkgs.nixpkgs-fmt ];
    } ''
    nixpkgs-fmt --check ${../flake.nix} ${../nix}
    touch $out # it worked!
  '';
}
