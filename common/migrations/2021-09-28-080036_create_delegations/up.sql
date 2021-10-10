-- Your SQL goes here
CREATE TABLE entities (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  actor_id VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  secret_key BLOB,
  public_key BLOB
);

CREATE TABLE roles (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  actor_id VARCHAR NOT NULL,
  entity_id VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  is_assignment BOOLEAN NOT NULL,
  secret_key BLOB,
  public_key BLOB
);

CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  actor_id VARCHAR NOT NULL,
  entity_id VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  secret_key BLOB,
  public_key BLOB
);

CREATE TABLE delegations (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    subject_id VARCHAR NOT NULL,
    object_id VARCHAR NOT NULL,
    issuer_id VARCHAR NOT NULL
);
