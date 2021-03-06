CREATE TABLE registrations(
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  username VARCHAR NOT NULL,
  domain VARCHAR NULL,
  contact VARCHAR NOT NULL,
  expires TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (NOW() + interval '1 hour'),
  call_id VARCHAR NOT NULL,
  cseq INTEGER NOT NULL,
  user_agent VARCHAR NOT NULL,
  instance VARCHAR NULL,
  reg_id INTEGER NOT NULL DEFAULT 0,
  ip_address INET NOT NULL,
  port SMALLINT NOT NULL,
  transport VARCHAR NOT NULL
);
SELECT diesel_manage_updated_at('registrations');
