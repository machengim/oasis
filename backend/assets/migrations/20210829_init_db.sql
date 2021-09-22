CREATE TABLE IF NOT EXISTS site (
	site_id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	version TEXT NOT NULL,
  	storage TEXT,
	secret TEXT,
	language TEXT NOT NULL,
	update_freq TEXT NOT NULL,
	updated_at INTEGER,
	created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS user (
	user_id INTEGER PRIMARY KEY,
	username TEXT NOT NULL UNIQUE,
  	password TEXT NOT NULL,
  	permission INTEGER NOT NULL DEFAULT 1,
	created_at INTEGER NOT NULL
);