table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        author -> Text,
        content -> Text,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}
