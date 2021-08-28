CREATE TABLE IF NOT EXISTS site (
	site_id INTEGER PRIMARY KEY,
	version REAL NOT NULL,
	first_run INTEGER NOT NULL DEFAULT 1,
  	storage TEXT,
	secret TEXT,
	created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS user (
	user_id INTEGER PRIMARY KEY,
	username TEXT NOT NULL UNIQUE,
  	password TEXT NOT NULL,
  	permission INTEGER NOT NULL DEFAULT 1,
	created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS file (
	file_id INTEGER PRIMARY KEY,
	filename TEXT NOT NULL,
	size INTEGER NOT NULL,
  	file_type TEXT,
  	owner_id INTEGER NOT NULL,
  	parent_id INTEGER NOT NULL,
  	last_modified_at INTEGER NOT NULL
);