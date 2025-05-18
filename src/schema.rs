// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int8,
        created_at -> Timestamptz,
        chat_id -> Int8,
        content -> Text,
        author_id -> Int8,
    }
}
