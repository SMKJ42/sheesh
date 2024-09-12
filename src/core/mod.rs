use std::error;

use chrono::{offset::LocalResult, DateTime, TimeDelta, Utc};

pub mod auth_token;
pub mod id;
pub mod session;
pub mod user;

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
