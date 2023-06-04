use {
    axum::{response::Json, routing::get, Router},
    diesel::{pg::PgConnection, prelude::*},
    dotenvy::dotenv,
    serde::Serialize,
    std::env,
};

mod schema;

pub fn db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::heroes)]
struct Hero {
    id: i32,
    name: String,
    hit_points: i32,
    attack: i32,
    defense: i32,
}

async fn get_heroes() -> Json<Vec<Hero>> {
    let heroes = schema::heroes::table
        .select(Hero::as_select())
        .get_results(&mut db())
        .unwrap();

    Json(heroes)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/heroes", get(get_heroes));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
