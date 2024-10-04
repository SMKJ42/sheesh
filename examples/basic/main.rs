mod types;

use sheesh::{harness::DbHarness, session::SessionManagerConfig, user::UserManagerConfig};

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
    let harness = DbHarness::new_sqlite(pool);

    // once the harness is selected, go ahead and initialize tables. Init tables creates a table if user, session and token tables do not already exist.
    harness.init_tables().unwrap();

    let user_manager = UserManagerConfig::default().init(harness.user);
    let session_manager = SessionManagerConfig::default().init(harness.session, harness.token);

    let mut i = 0;

    loop {
        let user: MyUser = user_manager
            .create_user(
                "test".to_string(),
                "pwd".to_string(),
                Roles::Admin.as_role(),
                Some(MyPublicUserMetadata),
                Some(MyPrivateUserMetadata),
            )
            .unwrap();

        let pwd_str = "pwd";

        match user_manager.login(&session_manager, &user, pwd_str) {
            Ok((mut session, refresh_secret, _access_secret)) => {
                // creating a new access token...
                let _access_secret = session_manager
                    .create_new_access_token(&mut session, user.id())
                    .unwrap();

                // creating a new sesson token...
                let (_refresh_secret, _access_secret) = session_manager
                    .create_new_refresh_token(session, user.id(), &refresh_secret)
                    .unwrap();

                match user_manager.logout(&session_manager, &user, &refresh_secret) {
                    Ok(()) => {}
                    Err(err) => {}
                }
            }
            Err(_err) => {}
        }

        println!("{}", i);
        i += 1;
    }
}
