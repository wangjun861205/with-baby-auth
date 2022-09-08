-- Your SQL goes here

CREATE TABLE IF NOT EXISTS accounts (
	id SERIAL NOT NULL,
	username VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
	salt VARCHAR NOT NULL,
	PRIMARY KEY (id),
	UNIQUE (username)
);