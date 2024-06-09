use std::{env, fmt::Debug, str::FromStr};

const GEONAMES_USERNAME: &str = "GEONAMES_USERNAME";

pub fn get_geoname_username(default: &str) -> String {
    get_value(GEONAMES_USERNAME, default)
}

fn get_value<F: FromStr>(key: &str, default: &str) -> F
where
    <F as FromStr>::Err: Debug,
{
    env::var(key)
        .unwrap_or_else(|_| default.to_owned())
        .parse::<F>()
        .unwrap_or_else(|_| {
            panic!("Unable to parse environment variable {key} or default value {default}")
        })
}
