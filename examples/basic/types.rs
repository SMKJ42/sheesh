use std::fmt::Display;

use sheesh::{
    harness::IntoValues,
    user::{Group, PrivateUserMeta, PublicUserMeta, Role, User},
};

use serde::Serialize;

// the following trait impls create type safety for you across the application.
pub enum Roles {
    Admin,
    User,
}

impl Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => {
                write!(f, "admin")
            }
            Self::User => {
                write!(f, "user")
            }
        }
    }
}

impl Role for Roles {}

pub struct MyPublicUserMetadata {}
impl PublicUserMeta for MyPublicUserMetadata {}
impl IntoValues for MyPublicUserMetadata {
    fn into_values(&self) -> Vec<String> {
        return vec![];
    }
}

pub struct MyPrivateUserMetadata {}
impl PrivateUserMeta for MyPrivateUserMetadata {}
impl IntoValues for MyPrivateUserMetadata {
    fn into_values(&self) -> Vec<String> {
        return vec![];
    }
}

pub struct SomeGroup {}
impl Group for SomeGroup {}
impl Serialize for SomeGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("some_group")
    }
}

impl Display for SomeGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "SomeGroup");
    }
}

pub type MyUser = User<Roles, SomeGroup, MyPublicUserMetadata, MyPrivateUserMetadata>;
