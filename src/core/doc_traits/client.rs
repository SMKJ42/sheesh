use std::error;

use crate::core::user::{Group, PrivateUserMeta, PublicUserMeta, Role, User, UserPublic};

pub trait Client {
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
}
