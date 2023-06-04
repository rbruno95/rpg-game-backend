pub mod get;

use crate::schema;

use {diesel::prelude::*, serde::Serialize};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::heroes)]
pub struct Hero {
    id: i32,
    name: String,
    hit_points: i32,
    attack: i32,
    defense: i32,
}
