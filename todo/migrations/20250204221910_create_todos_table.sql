-- Add migration script here
CREATE TABLE todo_items (
  id INTEGER PRIMARY KEY,
  item VARCHAR NOT NULL,
  done_at DATETIME,
  deleted_at DATETIME,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX todo_items_item ON todo_items(item);
