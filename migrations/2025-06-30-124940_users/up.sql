CREATE table users(
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL, 
    role_id  VARCHAR NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP, 
    deleted_at TIMESTAMP
    )