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

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 30]
        first_name -> Nullable<Varchar>,
        #[max_length = 30]
        last_name -> Nullable<Varchar>,
        #[max_length = 252]
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
