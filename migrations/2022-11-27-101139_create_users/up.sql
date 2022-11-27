CREATE TABLE "users" (
    id SERIAL PRIMARY KEY,
    username character varying(256) NOT NULL UNIQUE,
    password_salt character (12) NOT NULL UNIQUE,
    password_hash character varying(256) NOT NULL,
    created timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    updated timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
)

