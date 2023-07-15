use {
    diesel::{pg::PgConnection, prelude::*},
    lazy_static::lazy_static,
    std::env,
};

lazy_static! {
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL not found in the environment variables");
}

pub fn db() -> PgConnection {
    PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", *DATABASE_URL))
}
