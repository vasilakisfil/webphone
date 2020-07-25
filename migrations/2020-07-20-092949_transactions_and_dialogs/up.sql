CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE dialogs(
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  computed_id VARCHAR NOT NULL,
  call_id VARCHAR NOT NULL,
  from_tag VARCHAR NOT NULL,
  to_tag VARCHAR NOT NULL,
  flow VARCHAR NOT NULL
);
SELECT diesel_manage_updated_at('dialogs');

CREATE TABLE transactions(
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  state VARCHAR(255) NOT NULL,
  branch_id VARCHAR NOT NULL,
  dialog_id BIGINT NOT NULL REFERENCES dialogs(id) ON UPDATE CASCADE ON DELETE CASCADE
);
SELECT diesel_manage_updated_at('transactions');
