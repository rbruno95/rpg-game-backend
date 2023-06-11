use crate::{diesel_helpers::db, schema};

use super::Hero;

use {axum::Json, diesel::prelude::*};

pub async fn perform() -> Json<Vec<Hero>> {
    let heroes = schema::heroes::table
        .select(Hero::as_select())
        .get_results(&mut db())
        .unwrap();

    Json(heroes)
}
