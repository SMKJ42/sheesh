use std::fmt::Display;

use sheesh::user::{Group, PrivateUserMeta, PublicUserMeta, Role, User};

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
impl PublicUserMeta for MyPublicUserMetadata {
    fn from_values(values: Vec<String>) -> Self {
        Self {}
    }
    fn into_values(&self) -> Vec<String> {
        vec![]
    }
}

pub struct MyPrivateUserMetadata {}
impl PrivateUserMeta for MyPrivateUserMetadata {
    fn from_values(values: Vec<String>) -> Self {
        Self {}
    }
    fn into_values(&self) -> Vec<String> {
        vec![]
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
        return write!(f, "some_group");
    }
}

pub type MyUser = User<Roles, SomeGroup, MyPublicUserMetadata, MyPrivateUserMetadata>;
