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

## License

This project is licensed under the MIT license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion by you, shall be licensed as MIT, without any additional terms or
conditions.
