use crate::modules::structs::*;
use reqwest;
use reqwest::header::HeaderMap;
use std::env;

pub async fn authenticate(code: String) -> Result<DiscordAuthentication, reqwest::Error> {
    let client = reqwest::Client::new();
    let domain = env::var("DOMAIN").unwrap();
    let client_id = env::var("DISCORD_CLIENT_ID").unwrap();
    let client_secret = env::var("DISCORD_CLIENT_SECRET").unwrap();
    let callback_url = format!("{}/discord/callback", domain);
    let param = reqwest::multipart::Form::new()
        .text("client_id", client_id)
        .text("client_secret", client_secret)
        .text("grant_type", "authorization_code")
        .text("code", code)
        .text("scope", "identify")
        .text("redirect_uri", callback_url);

    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    let response = client
        .post("https://discord.com/api/oauth2/token")
        .multipart(param)
        .send()
        .await?;

    Ok(response.json::<DiscordAuthentication>().await?)
}

pub async fn users_me(token: String) -> Result<DiscordUserData, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let response = client
        .get("https://discord.com/api/v6/users/@me")
        .headers(headers)
        .send()
        .await?;

    Ok(response.json::<DiscordUserData>().await?)
}

pub async fn application_info() -> Result<ApplicationInfo, reqwest::Error> {
    let client = reqwest::Client::new();
    let token = env::var("DISCORD_BOT_TOKEN").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bot {}", token).parse().unwrap());

    let response = client
        .get("https://discord.com/api/v6/oauth2/applications/@me")
        .headers(headers)
        //.multipart(param)
        // .basic_auth(client_id, Some(client_secret))
        .send()
        .await?;

    Ok(response.json::<ApplicationInfo>().await?)
}
