use crate::schema;

use {chrono::NaiveDateTime, diesel::prelude::*, serde::Serialize};

pub mod get;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::characters)]
pub struct Character {
    id: i32,
    name: String,
    created_at: NaiveDateTime,
}
