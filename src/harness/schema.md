### The following SQL statments are the default SQL statements used when no extra fields are provided.

## User

username STRING,
session_id INTEGER,
id STRING,
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
token STRING,
expires DATETIME
