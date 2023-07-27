// @generated automatically by Diesel CLI.

diesel::table! {
    permissions (name) {
        name -> Text,
        flag -> Int8,
    }
}

diesel::table! {
    role_permissions (role, permission) {
        role -> Text,
        permission -> Text,
    }
}

diesel::table! {
    roles (name) {
        name -> Text,
    }
}

diesel::table! {
    todos (id) {
        id -> Uuid,
        title -> Text,
        content -> Text,
        created_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        username -> Text,
    }
}

diesel::table! {
    user_roles (username, role) {
        username -> Text,
        role -> Text,
    }
}

diesel::table! {
    users (username) {
        username -> Text,
        password_sha512 -> Bytea,
    }
}

diesel::joinable!(role_permissions -> permissions (permission));
diesel::joinable!(role_permissions -> roles (role));
diesel::joinable!(todos -> users (username));
diesel::joinable!(user_roles -> roles (role));
diesel::joinable!(user_roles -> users (username));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    role_permissions,
    roles,
    todos,
    user_roles,
    users,
);
