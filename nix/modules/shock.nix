{ config, pkgs, lib, ... }:

let
  inherit (lib)
    literalExpression
    mkOption
    types;

  cfg = config.services.shock;
  tomlFormat = pkgs.formats.toml { };

  defaultUser = "shock";

  mkSingleJob = name: cfgJob:
    let
      inherit (cfgJob) prefix;

      r = lib.optionalString cfgJob.recursive "-r";
      v = lib.optionalString cfgJob.verbose "-v";
      destroy = lib.optionalString cfgJob.destroy "--destroy";
      datasets = lib.strings.escapeShellArgs cfgJob.datasets;
      config = lib.strings.escapeShellArg (tomlFormat.generate "shock-config-${name}.toml" {
        inherit prefix;
      });
    in
    lib.optionalString (prefix != { })
      "${cfg.package}/bin/shock ${v} ${r} ${destroy} -c ${config} ${datasets}";

  mkZfsCmd = args: lib.escapeShellArgs ([ "/run/booted-system/sw/bin/zfs" ] ++ args);
  mkDelegateCmd = allow: cfgJob: lib.flip map (lib.optionals (cfgJob.destroy) cfgJob.datasets) (dataset:
    let
      exists = lib.optionalString
        (allow == "allow")
        "${mkZfsCmd [ "list" dataset ]} >/dev/null && ";

      d = lib.optionalString cfgJob.recursive "d";
      delegate = mkZfsCmd [
        allow
        "-l${d}u"
        cfg.user
        "destroy,mount"
        dataset
      ];
    in
    exists + delegate
  );

  mkDelegate = allow: pkgs.writeShellScript "shock-zfs-${allow}" (
    lib.concatStringsSep "\n" (lib.flatten (map (mkDelegateCmd allow) (lib.attrValues cfg.jobs)))
  );
in
{
  options.services.shock = {
    group = mkOption {
      type = types.str;
      default = defaultUser;
      description = "The group for the service.";
    };

    jobs = mkOption {
      default = { };
      description = "Attrset of shock jobs to run";
      type = types.attrsOf (types.submodule {
        options = {
          datasets = mkOption {
            type = types.listOf types.str;
            description = "Datasets to operate on";
          };

          destroy = mkOption {
            default = false;
            type = types.bool;
            example = true;
            description = "Peform destructive actions (deleting snapshots). Do a dry run if false";
          };

          prefix = mkOption {
            description = "Mapping of a snapshot name prefix, to how many such snapshots should be kept per dataset";
            example = literalExpression ''
              {
                zfs-auto-snap_monthly = 12;
                zfs-auto-snap_weekly = 4;
                zfs-auto-snap_daily = 7;
                zfs-auto-snap_hourly = 24;
                zfs-auto-snap_frequent = 4;
              }
            '';
          };

          recursive = mkOption {
            default = false;
            type = types.bool;
            example = true;
            description = "Whether to recursively operate on the provided datasets";
          };

          verbose = mkOption {
            default = false;
            type = types.bool;
            example = true;
            description = "Whether to enable verbose logging";
          };
        };
      });
    };

    package = mkOption {
      type = types.package;
      default = pkgs.shock;
      defaultText = "pkgs.shock";
      description = "Package providing shock";
    };

    persistentTimer = mkOption {
      default = false;
      type = types.bool;
      example = true;
      description = lib.mdDoc ''
        Set the `persistentTimer` option for the
        {manpage}`systemd.timer(5)`
        which triggers the cleanup immediately if the last trigger
        was missed (e.g. if the system was powered down).
      '';
    };

    startAt = mkOption {
      type = with types; either str (listOf str);
      default = "daily";
      description = lib.mdDoc ''
        When or how often the cleanup should run.
        Must be in the format described in
        {manpage}`systemd.time(7)`.
        If you do not want the service to start
        automatically, use `[ ]`.
      '';
    };

    user = mkOption {
      type = types.str;
      default = "shock";
      description = lib.mdDoc ''
        The user for the shock service. ZFS privilege delegation will be
        automatically configured for any local datasets configured for any job
        if this is set to a user other than root. The user will be given
        "destroy" privileges on the specified datasets (recursively if necessary).
        The privileges will be revoked after the service finishes running.
      '';
    };
  };

  config = lib.mkIf (cfg.jobs != { }) {
    systemd.timers.shock = {
      description = "shock timer";
      wantedBy = [ "timers.target" ];
      timerConfig = {
        Persistent = cfg.persistentTimer;
        OnCalendar = cfg.startAt;
      };
    };

    systemd.services.shock = {
      path = [ "/run/booted-system/sw" ];
      restartIfChanged = false;

      script = ''
        has_err=""
        ${lib.concatStringsSep "\n" (lib.mapAttrsToList
          (name: cfgJob: "${mkSingleJob name cfgJob} || has_err=1")
          cfg.jobs
        )};
        if [[ -n "$has_err" ]]; then
          echo "at least one job failed, check logs for details"
          exit 1
        fi
      '';

      serviceConfig = {
        CapabilityBoundingSet = "";
        DeviceAllow = [ "/dev/zfs" ];
        ExecStartPre = "-+${mkDelegate "allow"}";
        ExecStopPost = "-+${mkDelegate "unallow"}";
        Group = cfg.group;
        IPAddressDeny = "any";
        LockPersonality = true;
        MemoryDenyWriteExecute = true;
        NoNewPrivileges = true;
        PrivateDevices = false; # zfs list fails without this, not sure what other device is needed
        PrivateNetwork = true;
        PrivateTmp = true;
        PrivateUsers = true;
        ProcSubset = "pid";
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectProc = "invisible";
        ProtectSystem = "strict";
        RemoveIPC = true;
        RestrictAddressFamilies = "none";
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = "native";
        SystemCallFilter = [
          "~@clock"
          "~@cpu-emulation"
          "~@debug"
          "~@module"
          "~@mount"
          "~@obsolete"
          "~@privileged"
          "~@raw-io"
          "~@reboot"
          "~@resources"
          "~@swap"
        ];
        UMask = "0077";
        User = cfg.user;
        WorkingDirectory = "${cfg.package}";
      };
    };

    users = lib.optionalAttrs (cfg.user == defaultUser) {
      groups.${defaultUser} = { };
      users.${defaultUser} = {
        group = cfg.group;
        isSystemUser = true;
      };
    };
  };
}
