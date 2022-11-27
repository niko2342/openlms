// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_salt -> Bpchar,
        password_hash -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}
