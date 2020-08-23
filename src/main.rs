use actix_web::{web, App, HttpServer};
use kankyo;
use modules::{database, handlers, structs::*};
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use tera::Tera;

mod auth;
mod modules;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    let data = web::Data::new(Mutex::new(ApplicationData {
        statemap: HashMap::<String, UserData>::new(),
        database: database::get_pool(env::var("DATABASE_URL").unwrap().as_str())
            .await
            .unwrap(),
        tera: Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap(),
    }));

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/static/style.css", web::get().to(handlers::get_css))
            .route("/connect", web::get().to(handlers::start_flow))
            .route("/lastfm/auth", web::get().to(handlers::lastfm_auth))
            .route("/lastfm/callback", web::get().to(handlers::lastfm_callback))
            .route("/discord/auth", web::get().to(handlers::discord_auth))
            .route("/discord/callback", web::get().to(handlers::discord_callback))
            .route("/done", web::get().to(handlers::flow_complete))
            .external_resource(
                "lastfm_auth",
                "https://www.last.fm/api/auth?api_key={api_key}&cb={callback}",
            )
            .external_resource(
                "discord_auth", 
                "https://discord.com/api/oauth2/authorize?response_type=code&scope=identify&prompt=consent&redirect_uri={redirect}&client_id={client_id}&state={state}"
            )
    })
    .bind(env::var("BIND").unwrap())?
    .run()
    .await
}
