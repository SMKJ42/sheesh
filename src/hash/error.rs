use scrypt::password_hash::{self, rand_core};

pub enum Error {
    RandErr(RandErr),
    HashErr(HashErr),
}

// initial types, will be revised later, but this will hopefully prevent large breaking changes.
pub type RandErr = password_hash::rand_core::Error;
pub type HashErr = password_hash::Error;

impl From<rand_core::Error> for Error {
    fn from(err: password_hash::rand_core::Error) -> Self {
        Self::RandErr(err)
    }
}

impl From<password_hash::Error> for Error {
    fn from(err: password_hash::Error) -> Self {
        Self::HashErr(err)
    }
}
