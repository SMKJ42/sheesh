use std::error;

use crate::{
    auth_token::AuthToken,
    session::Session,
    user::{Group, PrivateUserMeta, PublicUserMeta, Role, User, UserPublic},
};

/// The ServerActions trait provides some default functions to interact with your database.
///
/// While this trait is not used internally, it can help reduce the cognitive overhead of
/// rolling your own authentication.
pub trait ServerActions {
    /// Retrieve data that is intended to be available to all users.
    fn query_user_public<Pu: PublicUserMeta>() -> Result<UserPublic<Pu>, Box<dyn error::Error>>;

    /// Retrieve data that is intended to be available only to the user.
    fn query_user_private<R: Role, G: Group, Pu: PublicUserMeta, Pr: PrivateUserMeta>(
    ) -> Result<User<R, G, Pu, Pr>, Box<dyn error::Error>>;

    /// Update user's role. Role should always contain a value.
    fn update_role(user_id: String) -> Result<(), Box<dyn error::Error>>;

    /// Add a group to a user's account.
    fn add_group(user_id: String) -> Result<(), Box<dyn error::Error>>;

    /// Remove a group from a user's account.
    fn remove_group(user_id: String) -> Result<(), Box<dyn error::Error>>;

    /// A function to create a new user in your database.
    fn insert_user<R: Role, G: Group, Pu: PublicUserMeta, Pr: PrivateUserMeta>(
        user: User<R, G, Pu, Pr>,
    ) -> Result<(), Box<dyn error::Error>>;

    /// A function to remove a user from your database.
    fn remove_user(user_id: String) -> Result<(), Box<dyn error::Error>>;

    //
    //
    //

    /// Create a new session in your database.
    fn insert_session(session: Session) -> Result<(), Box<dyn error::Error>>;

    /// Find a session in your database.
    fn query_session() -> Result<Session, Box<dyn error::Error>>;

    /// Create a new auth token for a user's session, while maintaining the current session.

    fn end_session(session_id: u64) -> Result<(), Box<dyn error::Error>>;

    //
    //
    //

    fn refresh_session_token() -> Result<AuthToken, Box<dyn error::Error>>;

    /// Create a new auth token for a user's session.
    fn insert_auth_token(token: AuthToken) -> Result<(), Box<dyn error::Error>>;

    /// Find a auth token in your database.
    fn query_auth_token<T>(token_id: T) -> Result<AuthToken, Box<dyn error::Error>>;

    /// Invalidate a session token.
    fn invalidate_token<T>(token_id: T) -> Result<(), Box<dyn error::Error>>;
}
