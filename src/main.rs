use {
    axum::{routing::get, Router},
    dotenvy::dotenv,
    lazy_static::lazy_static,
    std::env,
};

mod diesel_helpers;
mod heroes;
mod schema;

lazy_static! {
    static ref SERVICE_PATH: String = {
        dotenv().ok();
        env::var("SERVICE_PATH").expect("SERVICE_PATH must be set")
    };
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/heroes", get(heroes::get::perform));

    axum::Server::bind(&SERVICE_PATH.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
