use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_key() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(16).collect()
}
