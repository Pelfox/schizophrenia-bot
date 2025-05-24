// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Int8,
        created_at -> Timestamptz,
        chat_id -> Int8,
        image_id -> Text,
    }
}

diesel::table! {
    messages (id) {
        id -> Int8,
        created_at -> Timestamptz,
        chat_id -> Int8,
        content -> Text,
        author_id -> Int8,
    }
}

diesel::table! {
    stickers (id) {
        id -> Int8,
        created_at -> Timestamptz,
        chat_id -> Int8,
        sticker_id -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(images, messages, stickers,);
