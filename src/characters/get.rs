use crate::{diesel_helpers::db, schema};

use super::Character;

use {axum::Json, diesel::prelude::*};

pub async fn perform() -> Json<Vec<Character>> {
    let characters = schema::characters::table
        .select(Character::as_select())
        .get_results(&mut db())
        .unwrap();

    Json(characters)
}
