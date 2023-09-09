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
                    if snapshot.name.as_bytes().starts_with(p.as_bytes()) {
                        Some(remaining)
                    } else {
                        if verbose {
                            eprintln!("ignoring {snapshot}");
                        }
                        None
                    }
                })
                .map(|remaining| {
                    if *remaining == 0 {
                        true
                    } else {
                        *remaining -= 1;
                        false
                    }
                })
                .unwrap_or(false)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
}
