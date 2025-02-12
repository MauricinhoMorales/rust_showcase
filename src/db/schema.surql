-- Define the `users` table
DEFINE TABLE users SCHEMAFULL;

-- Define fields for the `users` table
DEFINE FIELD username ON users TYPE string;
DEFINE FIELD password ON users TYPE string;

-- Create a unique index on the `username` field
DEFINE INDEX username_index ON users COLUMNS username UNIQUE;