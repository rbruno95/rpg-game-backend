use {
    axum::{routing::get, Router},
    lazy_static::lazy_static,
    std::env,
};

mod characters;
mod diesel_helpers;
mod schema;

lazy_static! {
    static ref SERVICE_PATH: String =
        env::var("SERVICE_PATH").expect("SERVICE_PATH not found in the environment variables");
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/characters", get(characters::get::perform));

    axum::Server::bind(&SERVICE_PATH.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
