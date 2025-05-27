-- Migration number: 0002 	 2025-05-24T11:03:39.741Z

create table memos (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  parent_memo_id UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
  user_id UUID NOT NULL UNIQUE,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);

create index idx_memos_user_id on memos (user_id);
create index idx_memos_parent_memo_id on memos (parent_memo_id);