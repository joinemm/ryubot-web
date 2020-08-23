use crate::modules::structs::*;
use md5;
use reqwest;
use std::env;

pub async fn authenticate(token: &str) -> Result<LastfmAuthentication, reqwest::Error> {
    let client = reqwest::Client::new();
    let lastfm_api_key = env::var("LASTFM_TOKEN").unwrap();
    let param = &[
        ("api_key", lastfm_api_key.as_str()),
        ("method", "auth.getSession"),
        ("token", token),
    ];

    let response = client
        .get("https://ws.audioscrobbler.com/2.0")
        .query(param)
        .query(&[("api_sig", sign_call(param).as_str()), ("format", "json")])
        .send()
        .await?;

    Ok(response.json::<LastfmAuthentication>().await?)
}

pub async fn get_user(session_key: String) -> Result<LastfmUserData, reqwest::Error> {
    let client = reqwest::Client::new();
    let lastfm_api_key = env::var("LASTFM_TOKEN").unwrap();
    let param = &[
        ("api_key", lastfm_api_key.as_str()),
        ("method", "user.getInfo"),
        ("sk", session_key.as_str()),
    ];

    let response = client
        .get("https://ws.audioscrobbler.com/2.0")
        .query(param)
        .query(&[("api_sig", sign_call(param).as_str()), ("format", "json")])
        .send()
        .await?;

    let userinfo = response.json::<UserGetInfo>().await?;
    Ok(userinfo.user)
}

fn sign_call(params: &[(&str, &str)]) -> String {
    let mut signature_string = String::new();
    for param in params {
        signature_string.push_str(param.0);
        signature_string.push_str(param.1);
    }

    let lastfm_secret = env::var("LASTFM_SECRET").unwrap();
    signature_string.push_str(lastfm_secret.as_str());

    let signature = md5::compute(signature_string);
    return format!("{:x}", signature);
}
