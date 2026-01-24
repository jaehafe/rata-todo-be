// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
