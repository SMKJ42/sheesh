use std::error;

use chrono::{offset::LocalResult, DateTime, TimeDelta, Utc};

use scrypt::password_hash::{
    rand_core::OsRng, Encoding, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use scrypt::Scrypt;

pub mod auth_token;
pub mod id;
pub mod session;
pub mod user;

// using pub static mut declaration here is doable, but would require an unsafe block.
pub const DEFAULT_RNG_STR_FN: &'static dyn Fn() -> String = &|| {
    return SaltString::generate(OsRng).to_string();
};

pub const DEFAULT_HASH_FN: &'static dyn Fn(&str, &str) -> Result<String, Box<dyn error::Error>> =
    &|pwd: &str, salt: &str| {
        let salt = SaltString::from_b64(salt)?;
        let res = Scrypt.hash_password(pwd.as_bytes(), salt.as_salt())?;
        return Ok(res.to_string());
    };

// default implementation stores the salt inside the secret, preventing required storage of the salt in a seperat field.
pub const DEFAULT_VERIFY_TOKEN_FN: &'static dyn Fn(
    &str,
    &str,
) -> Result<(), Box<dyn error::Error>> = &|pwd: &str, hash: &str| {
    let hash = PasswordHash::parse(hash, Encoding::B64)?;
    Scrypt.verify_password(pwd.as_bytes(), &hash)?;
    return Ok(());
};

fn get_token_expiry(ttl: i64) -> Result<DateTime<Utc>, Box<dyn error::Error>> {
    let now = Utc::now();
    let time_delta = TimeDelta::minutes(ttl);
    let (new_time, rem) = now.time().overflowing_add_signed(time_delta);
    let rem = chrono::Days::new(rem as u64);
    let now_add_day = now.checked_add_days(rem);

    match now_add_day {
        Some(some_now_add_day) => {
            let expires = some_now_add_day.with_time(new_time);
            match expires {
                LocalResult::Single(expires) => {
                    return Ok(expires);
                }
                LocalResult::Ambiguous(expires, _) => Ok(expires),
                LocalResult::None => {
                    unimplemented!();
                }
            }
        }
        None => {
            unimplemented!();
        }
    }
}
