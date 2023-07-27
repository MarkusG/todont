CREATE TABLE users (
    username TEXT PRIMARY KEY,
    password_sha512 BYTEA NOT NULL
);

CREATE TABLE roles (
    name TEXT PRIMARY KEY
);

CREATE TABLE permissions (
    name TEXT PRIMARY KEY,
    flag BIGINT NOT NULL
);

CREATE TABLE role_permissions (
    role TEXT NOT NULL REFERENCES roles(name),
    permission TEXT NOT NULL REFERENCES permissions(name),
    CONSTRAINT role_permissions_pkey PRIMARY KEY (role, permission)
);

CREATE TABLE user_roles (
    username TEXT NOT NULL REFERENCES users(username),
    role TEXT NOT NULL REFERENCES roles(name),
    CONSTRAINT user_role_pkey PRIMARY KEY (username, role)
);

DELETE FROM todos;

ALTER TABLE todos
ADD COLUMN username TEXT NOT NULL REFERENCES users(username);
