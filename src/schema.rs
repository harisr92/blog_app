// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 225]
        title -> Varchar,
        body -> Text,
        #[max_length = 50]
        status -> Varchar,
    }
}
