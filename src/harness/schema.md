### The following values are the default SQL values when no extra fields are provided.

## User

id INTEGER,
username STRING,
secret STRING,
salt STRING,
session_id INTEGER,
role STRING,
groups STRING,
ban TINYINT,
FOREIGN KEY(session_id) REFERENCES session(id)

## Session

id INTEGER PRIMARY KEY,
user_id INTEGER,
refresh_token INTEGER,
auth_token INTEGER,
expires DATETIME,
FOREIGN KEY(user_id) REFERENCES user(id),
FOREIGN KEY(refresh_token) REFERENCES token(id)

## Token

id INTEGER PRIMARY KEY,
secret STRING,
salt STRING,
expires DATETIME
