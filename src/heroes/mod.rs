use crate::schema;

use {chrono::NaiveDateTime, diesel::prelude::*, serde::Serialize};

pub mod get;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::heroes)]
pub struct Hero {
    id: i32,
    name: String,
    hit_points: i32,
    attack: i32,
    defense: i32,
    created_at: NaiveDateTime,
}
