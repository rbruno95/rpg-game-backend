#[macro_use]
extern crate dotenv_codegen;

use {
    axum::{routing::get, Router},
    lazy_static::lazy_static,
};

mod diesel_helpers;
mod heroes;
mod schema;

lazy_static! {
    static ref SERVICE_PATH: &'static str = dotenv!("SERVICE_PATH");
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/heroes", get(heroes::get::perform));

    axum::Server::bind(&SERVICE_PATH.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
