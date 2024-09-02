use scrypt::password_hash::{self, rand_core, PasswordHasher, PasswordVerifier};

use crate::method::PasswordMethod;
use crate::rng::DefaultRng;

pub struct HashBrown<T> {
    method: PasswordMethod,
    rng: T,
}

pub struct DefaultHashBrown;

impl DefaultHashBrown {
    pub fn init() -> HashBrown<DefaultRng> {
        return HashBrown {
            method: PasswordMethod::scrypt(),
            rng: DefaultRng,
        };
    }
}

impl<T> HashBrown<T>
where
    T: rand_core::CryptoRngCore + Clone,
{
    pub fn new(method: PasswordMethod, rng: T) -> Self {
        return Self { method, rng };
    }

    pub fn create_salt(&self) -> password_hash::SaltString {
        return password_hash::SaltString::generate(self.rng.clone());
    }

    pub fn verity_password<'a>(
        &self,
        pwd: String,
        salt: &'a password_hash::SaltString,
    ) -> Result<(), password_hash::Error> {
        match &self.method {
            PasswordMethod::Scrypt(scrypt) => {
                scrypt.verify_password(pwd.clone().as_bytes(), &self.hash(pwd, &salt)?)
            }
        }
    }

    pub fn hash<'a>(
        &self,
        pwd: String,
        salt: &'a password_hash::SaltString,
    ) -> Result<password_hash::PasswordHash<'a>, password_hash::Error> {
        match &self.method {
            PasswordMethod::Scrypt(scrypt) => scrypt.hash_password(pwd.as_bytes(), salt),
        }
    }
}
