# shock

Regularly `shock` your ZFS pools to maintain good hygiene and prune stale
snapshots.

# Usage

```shell
nix run github:ipetkov/shock -- --verbose --recursive --config ./path/to/config.toml tank/persist
```

or:

```shell
nix shell github:ipetkov/shock
```

then:

```shell
shock --verbose --recursive --config ./path/to/config.toml tank/persist
```

Note that `shock` will perform a dry-run by default. No data will be deleted
unless `--destroy` is passed in.

## NixOS

```nix
{
  inputs.shock.url = "github:ipetkov/shock";

  outputs = { self, nixpkgs, shock }: {
    nixosConfigurations.host = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
        shock.nixosModules.default
        ({ config, pkgs, ... }: {
          nixpkgs.overlays = [ inputs.shock.overlays.default ];
          services.shock = {
            startAt = "daily";
            persistentTimer = true;
            jobs = {
              backups = {
                # Disable to only operate on the dataset itself
                # and not any of its children
                recursive = true;
                verbose = true; # Disable for quieter logs
                datasets = [
                  "tank/backups"
                  "tank/another"
                ];
                #destroy = true; # Uncomment to actually destroy data!
                prefix = {
                  zfs-auto-snap_monthly = 12;
                  zfs-auto-snap_weekly = 4;
                  zfs-auto-snap_daily = 7;
                  zfs-auto-snap_hourly = 24;
                  zfs-auto-snap_frequent = 4;
                };
              };
            };
          };
        })
      ];
    };
  };
}
```

# Reference

```
Shock your ZFS pools to maintain good hygeine

Usage: shock [OPTIONS] --config <CONFIG> [DATASETS]...

Arguments:
  [DATASETS]...  The pools or datasets to shock

Options:
  -r, --recursive        Recursively operate on the specified datasets
  -v, --verbose          Enable verbose output
      --destroy          Perform destructive actions. Omit for dry run
  -c, --config <CONFIG>  Path to the TOML configuration
  -h, --help             Print help
  -V, --version          Print version
```

## Configuration

```toml
# Keep up to N snapshots whose name starts with the specified prefix.
# Only snapshots within the same dataset will be counted, and any snapshots
# whose name does not match any prefix will be ignored.
[prefix]
zfs-auto-snap_monthly = 12
zfs-auto-snap_weekly = 4
zfs-auto-snap_daily = 7
zfs-auto-snap_hourly = 24
zfs-auto-snap_frequent = 4
```

## Why this exists

Snapshot creation and pruning are inherently intertwined to the point that
usually they are both done with the same tool. It can get difficult, however,
to manage snapshots created on another host (and replicated to the current host)
without actually running the snapshot creator on that pool.

As a more concrete example:
[`zfs-auto-snapshot`](https://github.com/bdrewery/zfstools#zfs-auto-snapshot) only
prunes snapshots while creating snapshots, so if one is replicating a dataset
whose snapshots are created by `zfs-auto-snapshot` there isn't a good way to
prune them (especially since on NixOS there is a single global configuration for
what snapshots are kept). [Sanoid](https://github.com/jimsalterjrs/sanoid)
allows for comprehensive management policies, except it, sadly, only knows how
to manage snapshots it has created (and the names used by `zfs-auto-snapshot`
differ). For the situations where is is impractical or infeasible to use
`sanoid`, `shock` exists as an easy way to bridge the gap.

## License

This project is licensed under the MIT license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, shall be licensed as MIT, without any additional terms or
conditions.
