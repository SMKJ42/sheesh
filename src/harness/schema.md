### The following values are the default SQL values when no extra fields are provided.

## User

id INTEGER PRIMARY KEY,
session_id INTEGER,
username STRING NOT NULL UNIQUE,
secret STRING NOT NULL,
ban TINYINT NOT NULL,
groups STRING NOT NULL,
role STRING NOT NULL,
FOREIGN KEY(session_id) REFERENCES sessions(id)

## Session

id INTEGER PRIMARY KEY,
user_id INTEGER NOT NULL,
refresh_token INTEGER NOT NULL UNIQUE,
access_token INTEGER NOT NULL UNIQUE,
FOREIGN KEY(user_id) REFERENCES user(id),
FOREIGN KEY(refresh_token) REFERENCES refresh_tokens(id),
FOREIGN KEY(auth_token) REFERENCES access_tokens(id);

---

CREATE INDEX IF NOT EXISTS idx_user_id ON sessions(user_id);

## Refresh Token

id INTEGER PRIMARY KEY,
user_id INTEGER NOT NULL,
secret STRING NOT NULL,
expires DATETIME NOT NULL,
valid BOOL NOT NULL,
FOREIGN KEY(user_id) REFERENCES users(id);

---

CREATE INDEX IF NOT EXISTS idx_user_id ON refresh_tokens(user_id);

## Access Token

id INTEGER PRIMARY KEY NOT NULL,
user_id INTEGER NOT NULL,
token STRING NOT NULL,
expires DATETIME NOT NULL,
valid BOOL NOT NULL,
FOREIGN KEY(user_id) REFERENCES users(id);

---

CREATE INDEX IF NOT EXISTS idx_user_id ON access_tokens(user_id);
