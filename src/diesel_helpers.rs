use {
    diesel::{pg::PgConnection, prelude::*},
    lazy_static::lazy_static,
};

lazy_static! {
    static ref DATABASE_URL: &'static str = dotenv!("DATABASE_URL");
}

pub fn db() -> PgConnection {
    PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", *DATABASE_URL))
}
