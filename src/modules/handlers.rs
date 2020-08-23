use super::structs::*;
use super::utils;
use crate::auth::{discord, lastfm};
use crate::modules::database;
use actix_web::{http::header, web, HttpRequest, HttpResponse, Result};
use log::info;
use std::env;
use std::fs;
use std::sync::Mutex;
use tera::Context;

pub async fn start_flow(data: web::Data<Mutex<ApplicationData>>) -> Result<HttpResponse> {
    let state = utils::random_key();
    let mut data = data.lock().unwrap();

    data.statemap.insert(
        state.clone(),
        UserData {
            lastfm_session_key: None,
            discord_auth_token: None,
        },
    );

    let domain = env::var("DOMAIN").unwrap();
    let url = format!("{}/discord/auth?state={}", domain, state);
    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

pub async fn flow_complete(
    datamutex: web::Data<Mutex<ApplicationData>>,
    web::Query(statedata): web::Query<StateData>,
) -> Result<HttpResponse> {
    let mut mutdata = datamutex.lock().unwrap();

    let userdata = match mutdata.statemap.get_mut(&statedata.state) {
        Some(value) => value,
        None => return Ok(HttpResponse::Ok().body("Could not find user from given state")),
    };

    let session_key = format!("{}", userdata.lastfm_session_key.as_ref().unwrap());

    let discord_user =
        discord::users_me(format!("{}", userdata.discord_auth_token.as_ref().unwrap()))
            .await
            .unwrap();

    let lastfm_user =
        lastfm::get_user(format!("{}", userdata.lastfm_session_key.as_ref().unwrap()))
            .await
            .unwrap();

    std::mem::drop(mutdata);
    let data = datamutex.lock().unwrap();

    match database::add_user_data(
        &data.database,
        discord_user.id.parse::<i64>().unwrap(),
        session_key,
        &lastfm_user,
    )
    .await
    {
        Ok(res) => info!("{:?}", res),
        Err(e) => return Ok(HttpResponse::Ok().body(format!("{:?}", e))),
    }

    let application_info = discord::application_info().await.unwrap();

    let mut ctx = Context::new();
    let discordavatar = format!(
        "https://cdn.discordapp.com/avatars/{}/{}",
        &discord_user.id, &discord_user.avatar
    );
    let application_icon = format!(
        "https://cdn.discordapp.com/avatars/{}/{}",
        &application_info.id, &application_info.icon
    );
    let discordname = format!("{}#{}", &discord_user.username, &discord_user.discriminator);
    ctx.insert("discord_icon", discordavatar.as_str());
    ctx.insert("discord_name", discordname.as_str());
    ctx.insert("bot_icon", application_icon.as_str());
    ctx.insert("lastfm_icon", &lastfm_user.image.last().unwrap().url);
    ctx.insert("lastfm_name", &lastfm_user.name);
    let body = data.tera.render("index.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

pub async fn lastfm_auth(
    req: HttpRequest,
    web::Query(statedata): web::Query<StateData>,
) -> Result<HttpResponse> {
    let domain = env::var("DOMAIN").unwrap();
    let callback_url = format!("{}/lastfm/callback?state={}", domain, statedata.state);
    let lastfm_api_key = env::var("LASTFM_TOKEN").unwrap();

    let url = req
        .url_for("lastfm_auth", &[lastfm_api_key, callback_url])
        .unwrap();
    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

pub async fn lastfm_callback(
    data: web::Data<Mutex<ApplicationData>>,
    web::Query(callback): web::Query<LastfmCallback>,
) -> Result<HttpResponse> {
    let session = lastfm::authenticate(callback.token.as_str()).await.unwrap();
    let mut data = data.lock().unwrap();

    let userdata = match data.statemap.get_mut(&callback.state) {
        Some(value) => value,
        None => return Ok(HttpResponse::Ok().body("Could not find user from given state")),
    };
    userdata.lastfm_session_key = Some(session.session.key);

    let domain = env::var("DOMAIN").unwrap();
    let url = format!("{}/done?state={}", domain, callback.state);
    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

pub async fn discord_auth(
    web::Query(statedata): web::Query<StateData>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let domain = env::var("DOMAIN").unwrap();
    let callback_url = format!("{}/discord/callback", domain);
    let client_id = env::var("DISCORD_CLIENT_ID").unwrap();

    let url = req
        .url_for("discord_auth", &[callback_url, client_id, statedata.state])
        .unwrap();

    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

pub async fn discord_callback(
    data: web::Data<Mutex<ApplicationData>>,
    web::Query(callback): web::Query<DiscordCallback>,
) -> Result<HttpResponse> {
    let session = discord::authenticate(callback.code).await.unwrap();
    let mut data = data.lock().unwrap();

    let userdata = match data.statemap.get_mut(&callback.state) {
        Some(value) => value,
        None => return Ok(HttpResponse::Ok().body("Could not find user from given state")),
    };
    userdata.discord_auth_token = Some(session.access_token);

    let domain = env::var("DOMAIN").unwrap();
    let url = format!("{}/lastfm/auth?state={}", domain, callback.state);

    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

pub async fn get_css() -> Result<HttpResponse> {
    let css = fs::read_to_string("./static/style.css").expect("Cannot read CSS file");
    Ok(HttpResponse::Ok().content_type("text/css").body(css))
}
