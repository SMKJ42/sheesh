mod types;

use sheesh::{harness::DbHarness, session::SessionManagerConfig, user::UserManagerConfig};

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
use r2d2_sqlite::SqliteConnectionManager;

use types::*;

const CLEAN_ERR: &str =
    "Could not clean up example database file. check ./exmple_db dir for artifacts";

fn main() {
    // cleanup instance on ctl + c
    ctrlc::set_handler(|| {
        std::fs::remove_file("example_db/sqlite.db").expect(CLEAN_ERR);
    })
    .expect(CLEAN_ERR);

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

    while i < 100 {
        let user: MyUser = user_manager
            .create_user(
                i.to_string(),
                "pwd".to_string(),
                Roles::Admin.as_role(),
                Some(MyPublicUserMetadata),
                Some(MyPrivateUserMetadata),
            )
            .unwrap();

        let pwd_str = "pwd";

        match user_manager.login(&session_manager, &user, pwd_str) {
            Ok((mut session, refresh_secret, _access_secret)) => {
                // creating a new access token
                let _access_secret = session_manager
                    .create_new_access_token(&mut session, user.id())
                    .unwrap();

                // creating a new sesson token
                let (_refresh_secret, _access_secret) = session_manager
                    .create_new_refresh_token(session, user.id(), &refresh_secret)
                    .unwrap();

                // invalidating an access token is perfromed through the session.
                session_manager.invalidate_access_token(session).unwrap();

                // creating an access token takes an id and a session
                // DANGER -- issueing a new access token does not perform a token validation step, that is left up to the developer to handle.
                let access_secret = session_manager
                    .create_new_access_token(&mut session, user.id())
                    .unwrap();

                // verify an access token
                // DANGER -- this is performed through the know session, but the client should be sending the token String to be provided to this function.
                let _is_valid_access_token = session_manager.verify_access_token(
                    session.access_token().unwrap(),
                    user.id(),
                    &access_secret,
                );
                // verify a refresh_token
                // DANGER -- this is performed through the know session, but the client should be sending the token String to be provided to this function.
                let _is_valid_refresh_token = session_manager.verify_session_token(
                    session.refresh_token().unwrap(),
                    user.id(),
                    &refresh_secret,
                );

                // logout a user, this requires the user to know the refresh_secret. this prevents DOS
                // if you want functionality that logs out the user (for security reasons, not user request) try session_manager.invalidate_session()
                match user_manager.logout(&session_manager, &user, &refresh_secret) {
                    Ok(()) => {}
                    Err(_err) => {}
                }
            }
            Err(_err) => {}
        }

        println!("{}", i);
        i += 1;
    }

    // cleanup db instance
    std::fs::remove_file("example_db/sqlite.db").expect(CLEAN_ERR);
}
