use rusqlite::Connection;
use sheesh::{
    harness::sqlite::{SqliteDiskOpSession, SqliteDiskOpToken, SqliteDiskOpUser},
    session::{SessionManager, SessionManagerConfig},
    user::{Group, PrivateUserMeta, PublicUserMeta, Role, User, UserManager, UserManagerConfig},
};

enum Roles {
    Admin,
}

impl Role for Roles {}

struct MyPublicUserMetadata {}
impl PublicUserMeta for MyPublicUserMetadata {}

struct MyPrivateUserMetadata {}
impl PrivateUserMeta for MyPrivateUserMetadata {}

struct SomeGroup {}
impl Group for SomeGroup {}

type MyUser = User<Roles, SomeGroup, MyPublicUserMetadata, MyPrivateUserMetadata>;

fn main() {
    let connection: Connection = Connection::open_in_memory().unwrap();

    let user_manager = UserManagerConfig::new_default().init(SqliteDiskOpUser::new(connection));

    let session_manager = SessionManagerConfig::new_default().init(
        SqliteDiskOpSession::new(connection),
        SqliteDiskOpToken::new(connection),
    );

    let mut i = 0;

    loop {
        let mut user: MyUser = user_manager.new_user(
            "test".to_string(),
            Roles::Admin,
            MyPublicUserMetadata {},
            MyPrivateUserMetadata {},
        );

        let public = MyPublicUserMetadata {};
        let private = MyPrivateUserMetadata {};

        user.set_public_data(public);
        user.set_private_data(private);

        let (mut session, mut token) = session_manager.new_session(user.public().id()).unwrap();

        token.invalidate();

        let mut new_token = session_manager.refresh_session_token(&mut session).unwrap();

        let is_valid_token = new_token.is_valid();
        if i % 100 == 0 {
            println!("one loop");
        }
        i += 1;
    }
}
