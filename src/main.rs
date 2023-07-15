#[macro_use]
extern crate dotenv_codegen;

use {
    axum::{routing::get, Router},
    lazy_static::lazy_static,
};

mod characters;
mod diesel_helpers;
mod schema;

lazy_static! {
    static ref SERVICE_PATH: &'static str = dotenv!("SERVICE_PATH");
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/characters", get(characters::get::perform));

    axum::Server::bind(&SERVICE_PATH.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
