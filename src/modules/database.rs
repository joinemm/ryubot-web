use super::structs::*;
use sqlx::postgres::{PgDone, PgPool, PgPoolOptions};
use std::error::Error;

pub async fn get_pool(database_url: &str) -> Result<PgPool, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn add_user_data(
    pool: &PgPool,
    user_id: i64,
    lastfm_session_key: String,
    lastfm_user: &LastfmUserData,
) -> Result<PgDone, Box<dyn Error>> {
    let ret = sqlx::query!(
        "INSERT INTO lastfm_auth (user_id, lastfm_session, lastfm_username)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id)
        DO 
            UPDATE SET lastfm_session  = EXCLUDED.lastfm_session, 
                       lastfm_username = EXCLUDED.lastfm_username",
        user_id,
        lastfm_session_key,
        lastfm_user.name.clone()
    )
    .execute(pool)
    .await?;

    Ok(ret)
}
