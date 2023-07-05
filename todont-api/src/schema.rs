// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Uuid,
        title -> Text,
        content -> Nullable<Text>,
        created_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
    }
}
