use {
    diesel::{pg::PgConnection, prelude::*},
    dotenvy::dotenv,
    lazy_static::lazy_static,
    std::env,
};

lazy_static! {
    static ref DATABASE_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };
}

pub fn db() -> PgConnection {
    PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", *DATABASE_URL))
}
