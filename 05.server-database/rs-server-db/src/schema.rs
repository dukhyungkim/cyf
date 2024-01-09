// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Int4,
        title -> Text,
        url -> Text,
        alt_text -> Nullable<Text>,
    }
}
