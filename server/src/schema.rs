table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        title -> Text,
        content -> Text,
        date -> Datetime,
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

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
