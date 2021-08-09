CREATE TABLE IF NOT EXISTS site (
	site_id INTEGER PRIMARY KEY,
	version REAL NOT NULL,
	first_run INTEGER NOT NULL DEFAULT 1,
  	storage TEXT,
	secret TEXT,
	created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user (
	user_id INTEGER PRIMARY KEY,
	username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  permission INTEGER NOT NULL DEFAULT 1,
	created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS node (
	node_id INTEGER PRIMARY KEY,
	node_name TEXT NOT NULL,
  is_dir INTEGER NOT NULL DEFAULT 0,
  owner_id INTEGER NOT NULL,
  parent_node_id INTEGER,
	created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO site(version, created_at) VALUES(0.1, datetime());