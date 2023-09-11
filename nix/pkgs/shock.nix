{ craneLib
}:

let
  src = craneLib.cleanCargoSource ../..;

  baseArgs = {
    inherit src;
  };

  commonArgs = baseArgs // {
    cargoArtifacts = craneLib.buildDepsOnly baseArgs;
  };

  shock = craneLib.buildPackage commonArgs;

  flakeChecks = {
    clippy = craneLib.cargoClippy (commonArgs // {
      cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      doInstallCargoArtifacts = false;
    });

    fmt = craneLib.cargoFmt {
      inherit src;
    };
  };
in
shock // { inherit flakeChecks; }
