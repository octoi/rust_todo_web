-- Todo status enum
CREATE TYPE todo_status_enum AS ENUM (
  'open',
  'close'
);

-- Todo
CREATE TABLE todo (
  id BIGSERIAL,
  cid BIGINT NOT NULL, -- creator user id
	ctime TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  mid BIGINT, -- modifier user id
	mtime TIMESTAMP WITH TIME ZONE,   
  title TEXT NOT NULL,
  status todo_status_enum NOT NULL DEFAULT 'open'
);

CREATE TABLE person (
  id BIGSERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);

ALTER SEQUENCE todo_id_seq RESTART WITH 1000;