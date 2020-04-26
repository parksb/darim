table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        content -> Text,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        avatar_url -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
