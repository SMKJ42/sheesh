:warning: :warning: This library is still in development and WILL break :warning: :warning:

Ever wanted to host your own authentication? No? well nobody really WANTs to. But just in case you HAVE to, here you go.

Sheesh-auth does not follow any particular protocol, roast me.

User passwords and refresh token secrets stored salted and hashed while access tokens are stored as plaintext.

User data is extensible.

Plug and play or custom hashing, salting, and id generating strategies.

Plug and play or custom db harness for easy integration into a SQLite database.

`cargo run --example basic --release`
