mod types;

use sheesh::{harness::DiskOpManager, session::SessionManagerConfig, user::UserManagerConfig};

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
use r2d2_sqlite::SqliteConnectionManager;

use types::*;

fn main() {
    // establish db connection.
    let conn_manager = SqliteConnectionManager::file("example_db/sqlite.db");
    let pool = r2d2::Pool::new(conn_manager).unwrap();

    // initalize db harness. if you would like to see how to implement your own, look inside the harness modules.
    let disk_manager = DiskOpManager::new_sqlite(pool);
    disk_manager.init_tables().unwrap();

    // provide db harness to function interfaces.
    let user_manager = UserManagerConfig::new_default().init(disk_manager.user);
    let session_manager =
        SessionManagerConfig::new_default().init(disk_manager.session, disk_manager.token);

    let mut i = 0;

    loop {
        let mut user: MyUser = user_manager
            .new_user(
                "test".to_string(),
                "pwd".to_string(),
                Roles::Admin,
                MyPublicUserMetadata {},
                MyPrivateUserMetadata {},
            )
            .unwrap();

        let public = MyPublicUserMetadata {};
        let private = MyPrivateUserMetadata {};

        user.set_public_data(public);
        user.set_private_data(private);

        let (mut session, mut auth_token, session_token) =
            session_manager.new_session(user.id()).unwrap();

        // let mut new_token = session_manager.refresh_session_token(&mut session).unwrap();

        println!("{}", i);
        i += 1;
    }
}
