use std::error::Error;

use reqwest::Url;

use crate::config::get_geoname_username;

const BASE_URL: &'static str = "http://api.geonames.org/searchJSON";

pub async fn search_place(name: &str) -> Result<(), Box<dyn Error>> {
    let url = create_url(name)?;
    let res = reqwest::get(url).await?;
    println!("holidays = {:#?}", res.text().await.unwrap());
    Ok(())
}

fn create_url(name: &str) -> Result<Url, Box<dyn Error>> {
    let username = get_geoname_username("demo");
    let params = [
        ("username", username.as_str()),
        ("style", "short"),
        ("name", name),
    ];
    Ok(reqwest::Url::parse_with_params(&BASE_URL, &params)?)
}
