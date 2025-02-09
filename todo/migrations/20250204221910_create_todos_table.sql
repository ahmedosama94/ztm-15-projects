-- Add migration script here
CREATE TABLE todo_lists (
  id INTEGER PRIMARY KEY,
  title VARCHAR NOT NULL,
  deleted_at DATETIME,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX todo_lists_title_idx ON todo_lists(title);

CREATE TABLE todo_list_items (
  id INTEGER PRIMARY KEY,
  title VARCHAR NOT NULL,
  todo_list_id INTEGER NOT NULL,
  done_at DATETIME,
  deleted_at DATETIME,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX todo_list_items_todo_list_id ON todo_list_items(todo_list_id);
