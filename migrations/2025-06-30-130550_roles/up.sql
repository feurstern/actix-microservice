-- Your SQL goes here
CREATE TABLE roles(
 id  SERIAL PRIMARY  KEY,
 name VARCHAR NOT NULL,
 description VARCHAR,
 created_at TIMESTAMP,
 deleted_at TIMESTAMP
)