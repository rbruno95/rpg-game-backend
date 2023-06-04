// @generated automatically by Diesel CLI.

diesel::table! {
    heroes (id) {
        id -> Int4,
        name -> Varchar,
        hit_points -> Int4,
        attack -> Int4,
        defense -> Int4,
        created_at -> Timestamp,
    }
}
