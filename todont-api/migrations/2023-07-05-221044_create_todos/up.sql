CREATE TABLE todos (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT,
    created_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP
)
