use serde_derive::Deserialize;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use tera::Tera;

#[derive(Deserialize)]
pub struct LastfmCallback {
    pub token: String,
    pub state: String,
}

#[derive(Deserialize)]
pub struct StateData {
    pub state: String,
}

#[derive(Deserialize)]
pub struct DiscordCallback {
    pub code: String,
    pub state: String,
}

#[derive(Debug)]
pub struct UserData {
    pub lastfm_session_key: Option<String>,
    pub discord_auth_token: Option<String>,
}

#[derive(Debug)]
pub struct ApplicationData {
    pub statemap: HashMap<String, UserData>,
    pub database: PgPool,
    pub tera: Tera,
}

#[derive(Deserialize)]
pub struct LastfmAuthentication {
    pub session: AuthSession,
}
#[derive(Deserialize)]
pub struct DiscordAuthentication {
    pub token_type: String,
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct AuthSession {
    pub key: String,
    pub name: String,
}
#[derive(Deserialize, Debug)]
pub struct DiscordUserData {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
}

#[derive(Deserialize, Debug)]
pub struct LastfmImage {
    #[serde(rename = "#text")]
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct UserGetInfo {
    pub user: LastfmUserData,
}

#[derive(Deserialize, Debug)]
pub struct LastfmUserData {
    pub name: String,
    pub url: String,
    pub image: Vec<LastfmImage>,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationInfo {
    pub id: String,
    pub name: String,
    pub icon: String,
}
