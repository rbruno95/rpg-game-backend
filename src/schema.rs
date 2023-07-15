// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        created_at -> Timestamptz,
    }
}
