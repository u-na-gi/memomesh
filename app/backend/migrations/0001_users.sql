-- Migration number: 0001 	 2025-05-24T11:02:48.987Z

create table users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id UUID NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create index idx_users_user_id on users (user_id);
create index idx_users_email on users (email);