CREATE TABLE users(
  user_id UUID PRIMARY KEY,
  version UUID NOT NULL,
  created TIMESTAMP WITH TIME ZONE NOT NULL,
  updated TIMESTAMP WITH TIME ZONE NOT NULL,

  email TEXT NOT NULL,
  display_name TEXT NOT NULL,
  authentications JSONB NOT NULL
);