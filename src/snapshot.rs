use crate::config::Config;
use bstr::ByteSlice;
use std::{collections::HashMap, ffi::OsStr, fmt, os::unix::prelude::OsStrExt, path::Path};

pub struct Snapshot<'a> {
    dataset: &'a OsStr,
    name: &'a OsStr,
    whole: &'a OsStr,
}

impl fmt::Display for Snapshot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Path::new(self.whole).display().fmt(f)
    }
}

impl AsRef<OsStr> for Snapshot<'_> {
    fn as_ref(&self) -> &OsStr {
        self.whole
    }
}

pub fn to_delete<'a>(
    verbose: bool,
    config: &Config,
    zfs_list_output: &'a [u8],
) -> impl Iterator<Item = Snapshot<'a>> {
    let mut counts = HashMap::new();

    zfs_list_output
        .lines()
        .map(OsStr::from_bytes)
        .filter_map(|whole| {
            let ret = whole
                .as_bytes()
                .split_once_str("@")
                .map(|(dataset, name)| Snapshot {
                    whole,
                    dataset: OsStr::from_bytes(dataset),
                    name: OsStr::from_bytes(name),
                });

            if ret.is_none() {
                eprintln!("malformed output from `zfs list`: {:?}", whole);
            }

            ret
        })
        .filter(|snapshot| {
            counts
                .entry(snapshot.dataset)
                .or_insert_with(|| {
                    config
                        .prefix_configs
                        .iter()
                        .map(|p| (&p.prefix, p.keep))
                        .collect::<Vec<_>>()
                })
                .iter_mut()
                .find_map(|(p, remaining)| {
                    snapshot
                        .name
                        .as_bytes()
                        .starts_with(p.as_bytes())
                        .then_some(remaining)
                })
                .map(|remaining| {
                    if *remaining == 0 {
                        true
                    } else {
                        *remaining -= 1;
                        false
                    }
                })
                .unwrap_or_else(|| {
                    if verbose {
                        eprintln!("ignoring {snapshot}");
                    }
                    false
                })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
}

#[cfg(test)]
mod test {
    use super::to_delete;
    use crate::config::{Config, PrefixConfig};

    #[test]
    fn smoke() {
        assert_eq!(
            [
                "pool/data/nested@daily-1",
                "pool/data/nested@daily-2",
                "pool/data/nested@frequently-1",
                "pool/data/nested@frequently-2",
                "pool/data/nested@weekly-1",
                "pool/data/nested@weekly-2",
                "pool/data/nested@weekly-3",
                "tank/data@frequently-1",
                "tank/data@frequently-2",
                "tank/data/nested@frequently-1",
                "tank/data/nested@frequently-2",
            ]
            .as_slice(),
            to_delete(
                false,
                &Config::new(vec![
                    PrefixConfig {
                        prefix: "monthly".into(),
                        keep: 3,
                    },
                    PrefixConfig {
                        prefix: "weekly".into(),
                        keep: 0,
                    },
                    PrefixConfig {
                        prefix: "daily".into(),
                        keep: 2,
                    },
                    PrefixConfig {
                        prefix: "hourly".into(),
                        keep: 5,
                    },
                    PrefixConfig {
                        prefix: "frequently".into(),
                        keep: 1,
                    },
                ])
                .unwrap(),
                [
                    "tank/data/nested@frequently-3",
                    "tank/data/nested@hourly-1",
                    "tank/data/nested@frequently-2",
                    "tank/data/nested@frequently-1",
                    "tank/data@frequently-3",
                    "tank/data@frequently-2",
                    "tank/data@frequently-1",
                    "pool/data/nested@weekly-3",
                    "pool/data/nested@weekly-2",
                    "pool/data/nested@weekly-1",
                    "pool/data/nested@monthly-3",
                    "pool/data/nested@daily-4",
                    "pool/data/nested@frequently-3",
                    "pool/data/nested@frequently-2",
                    "pool/data/nested@monthly-2",
                    "pool/data/nested@daily-3",
                    "pool/data/nested@frequently-1",
                    "pool/data/nested@monthly-1",
                    "pool/data/nested@daily-2",
                    "pool/data/nested@daily-1",
                ]
                .join("\n")
                .as_bytes(),
            )
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
        );
    }
}
