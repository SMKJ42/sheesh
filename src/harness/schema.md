# User

username: String
id: String
role:
group(s):
ban: bool
session: Option(session_id)
id_token: Option(token_id)

# Session

id: i64 @unique
user_id: String
expires: DateTime
refresh_token: Option(token_id)

# Token

id: i64 @unique
token: String
user_id: String
expires: DateTime
