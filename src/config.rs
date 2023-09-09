#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixConfig {
    pub prefix: String,
    pub keep: u32,
}

#[allow(clippy::manual_non_exhaustive)] // NB: want to enforce this within the crate
#[derive(Debug)]
pub struct Config {
    pub datasets: Vec<String>,
    pub prefix_configs: Vec<PrefixConfig>,
    _priv: (),
}

impl Config {
    pub fn new(
        datasets: Vec<String>,
        mut prefix_configs: Vec<PrefixConfig>,
    ) -> Result<Self, Vec<(String, String)>> {
        prefix_configs.sort_by(|a, b| a.prefix.cmp(&b.prefix));

        let overlapping_prefixes = prefix_configs
            .iter()
            .map(|p| &p.prefix)
            .enumerate()
            .skip(1)
            .flat_map(|(i, cur)| {
                prefix_configs
                    .iter()
                    .map(|p| &p.prefix)
                    .take(i)
                    .filter(|prev| cur.starts_with(*prev))
                    .map(|prev| (prev.clone(), cur.clone()))
            })
            .collect::<Vec<_>>();

        if overlapping_prefixes.is_empty() {
            Ok(Self {
                datasets,
                prefix_configs,
                _priv: (),
            })
        } else {
            Err(overlapping_prefixes)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detects_overlaps() {
        let expected_datasets = vec!["pool/data/sub".into(), "tank/something/else".into()];
        let expected_prefix_configs = vec![
            PrefixConfig {
                prefix: "foo/bar".into(),
                keep: 42,
            },
            PrefixConfig {
                prefix: "foo/baz".into(),
                keep: 5,
            },
        ];

        let Config {
            datasets,
            prefix_configs,
            _priv,
        } = Config::new(expected_datasets.clone(), expected_prefix_configs.clone()).unwrap();

        assert_eq!(expected_datasets, datasets);
        assert_eq!(expected_prefix_configs, prefix_configs);

        let expected_err = [
            ("baz", "baz/asdf"),
            ("foo", "foo/baz"),
            ("foo", "foo/baz/qux"),
            ("foo/baz", "foo/baz/qux"),
        ]
        .map(|(a, b)| (a.into(), b.into()));

        assert_eq!(
            expected_err.as_slice(),
            Config::new(
                expected_datasets,
                vec![
                    PrefixConfig {
                        prefix: "foo".into(),
                        keep: 42,
                    },
                    PrefixConfig {
                        prefix: "foo/baz".into(),
                        keep: 5,
                    },
                    PrefixConfig {
                        prefix: "foo/baz/qux".into(),
                        keep: 5,
                    },
                    PrefixConfig {
                        prefix: "bar".into(),
                        keep: 5,
                    },
                    PrefixConfig {
                        prefix: "baz/asdf".into(),
                        keep: 5,
                    },
                    PrefixConfig {
                        prefix: "baz".into(),
                        keep: 5,
                    },
                ]
            )
            .unwrap_err()
        )
    }
}
