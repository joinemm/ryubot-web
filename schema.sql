CREATE TABLE IF NOT EXISTS lastfm_auth (
    user_id BIGINT PRIMARY KEY,
    lastfm_session VARCHAR (32),
    lastfm_username VARCHAR (32)
);