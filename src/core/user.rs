use std::{error, fmt::Display};

use serde::Serialize;

use crate::harness::{sqlite::SqliteDiskOpUser, DiskOpUser};

use super::{
    id::{DefaultIdGenerator, IdGenerator},
    DEFAULT_HASH_FN, DEFAULT_RNG_STR_FN, DEFAULT_VERIFY_TOKEN_FN,
};

// Traits that provide type safety for valid inputs.
pub trait Role {}
pub trait Group {}
pub trait PublicUserMeta {
    fn into_values(&self) -> Vec<String>;
    fn from_values(values: Vec<String>) -> Self;
}
pub trait PrivateUserMeta {
    fn into_values(&self) -> Vec<String>;
    fn from_values(values: Vec<String>) -> Self;
}

pub struct UserManagerConfig<'a, T>
where
    T: IdGenerator,
{
    id_generator: T,
    salt_fn: &'a dyn Fn() -> String,
    hash_fn: &'a dyn Fn(&str, &str) -> Result<String, Box<dyn error::Error>>,
    verify_pass_fn: &'a dyn Fn(&str, &str) -> Result<(), Box<dyn error::Error>>,
}

impl<'a> UserManagerConfig<'a, DefaultIdGenerator> {
    pub fn default() -> Self {
        Self {
            id_generator: DefaultIdGenerator {},
            salt_fn: DEFAULT_RNG_STR_FN,
            hash_fn: DEFAULT_HASH_FN,
            verify_pass_fn: DEFAULT_VERIFY_TOKEN_FN,
        }
    }
}

impl<'a, T> UserManagerConfig<'a, T>
where
    T: IdGenerator + Copy,
{
    pub fn init<V: DiskOpUser>(&self, harness: V) -> UserManager<T, V> {
        UserManager {
            id_generator: self.id_generator,
            hash_fn: self.hash_fn,
            verify_pass_fn: self.verify_pass_fn,
            salt_fn: self.salt_fn,
            harness,
        }
    }

    pub fn with_id_gen<X: IdGenerator + Copy>(&self, id_generator: X) -> UserManagerConfig<X> {
        return UserManagerConfig {
            id_generator,
            salt_fn: self.salt_fn,
            verify_pass_fn: self.verify_pass_fn,
            hash_fn: self.hash_fn,
        };
    }
}

pub struct UserManager<'a, T, V>
where
    T: IdGenerator,
    V: DiskOpUser,
{
    id_generator: T,
    harness: V,
    salt_fn: &'a dyn Fn() -> String,
    hash_fn: &'a dyn Fn(&str, &str) -> Result<String, Box<dyn error::Error>>,
    verify_pass_fn: &'a dyn Fn(&str, &str) -> Result<(), Box<dyn error::Error>>,
}

impl<'a> UserManager<'a, DefaultIdGenerator, SqliteDiskOpUser> {}

impl<'a, T, V> UserManager<'a, T, V>
where
    T: IdGenerator,
    V: DiskOpUser,
{
    pub fn create_user<R, G, Pu, Pr>(
        &self,
        user_name: String,
        pwd: String,
        role: R,
        public: Option<Pu>,
        private: Option<Pr>,
    ) -> Result<User<R, G, Pu, Pr>, Box<dyn error::Error>>
    where
        R: Role + Display,
        G: Group + Serialize,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        let id = self.id_generator.new_u64();

        let hash_fn = self.hash_fn;
        let salt_fn = self.salt_fn;

        let salt = salt_fn();
        let secret = hash_fn(&pwd, &salt)?;

        let user = User::new(id, user_name, secret, salt, role, public, private)?;

        self.harness.insert(&user)?;
        return Ok(user);
    }

    pub fn verify_pwd<R, G, Pu, Pr>(
        &self,
        user: User<R, G, Pu, Pr>,
        pwd: &str,
    ) -> Result<(), Box<dyn error::Error>>
    where
        R: Role + Display,
        G: Group + Serialize,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        let verify = self.verify_pass_fn;
        verify(pwd, &user.secret)
    }

    pub fn update_user<R, G, Pu, Pr>(
        &self,
        user: User<R, G, Pu, Pr>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        R: Role + Display,
        G: Group + Serialize,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        unimplemented!()
        // return self.harness.update(item, cols);
    }

    pub fn load_user<R, G, Pu, Pr>(
        &self,
        id: i64,
    ) -> Result<User<R, G, Pu, Pr>, Box<dyn error::Error>>
    where
        R: Role + Display,
        G: Group + Serialize,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        return self.harness.read(id);
    }

    pub fn delete_user<R, G, Pu, Pr>(&self, id: i64) -> Result<(), Box<dyn error::Error>>
    where
        R: Role + Display,
        G: Group + Serialize,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        return self.harness.delete(id);
    }
}

#[derive(Clone)]
pub struct User<R, G, Pu, Pr>
where
    R: Role,
    G: Group,
    Pu: PublicUserMeta,
    Pr: PrivateUserMeta,
{
    id: u64,
    user_name: String,
    secret: String,
    salt: String,
    role: R,
    groups: Vec<G>,
    public: Option<Pu>,
    private: Option<Pr>,
    ban: bool,
    session: Option<i64>,
}

impl<R, G, Pu, Pr> User<R, G, Pu, Pr>
where
    R: Role,
    G: Group,
    Pu: PublicUserMeta,
    Pr: PrivateUserMeta,
{
    pub fn new(
        id: u64,
        user_name: String,
        secret: String,
        salt: String,
        role: R,
        public: Option<Pu>,
        private: Option<Pr>,
    ) -> Result<Self, Box<dyn error::Error>> {
        return Ok(Self {
            id,
            user_name,
            secret,
            salt,
            role,
            groups: Vec::new(),
            public,
            private,
            ban: false,
            session: None,
        });
    }

    pub fn id(&self) -> u64 {
        return self.id;
    }

    pub fn user_name(&self) -> String {
        return self.user_name.clone();
    }

    pub fn update_user_name(&mut self, user_name: String) {
        self.user_name = user_name
    }

    pub fn role(&self) -> &R {
        return &self.role;
    }
    pub fn update_role(&mut self, role: R) {
        self.role = role
    }
    pub fn public(&self) -> &Option<Pu> {
        return &self.public;
    }
    pub fn private(&self) -> &Option<Pr> {
        return &self.private;
    }

    pub fn ban(&mut self) {
        self.ban = true;
    }

    pub fn unban(&mut self) {
        self.ban = false;
    }

    pub fn is_banned(&self) -> bool {
        return self.ban;
    }

    pub fn set_public_data(&mut self, public: Option<Pu>) {
        self.public = public
    }

    pub fn set_private_data(&mut self, private: Option<Pr>) {
        self.private = private;
    }

    pub fn groups(&self) -> Vec<G>
    where
        G: Clone,
    {
        return self.groups.to_owned();
    }

    pub fn from_values(values: Vec<String>) -> Self {
        unimplemented!()
    }

    pub fn into_values(&self) -> Vec<String> {
        unimplemented!()
    }

    pub fn salt(&self) -> String {
        return self.salt.to_owned();
    }

    pub fn session(&self) -> Option<i64> {
        return self.session;
    }
}

impl<R: Role, G: Group + PartialEq, Pu: PublicUserMeta, Pr: PrivateUserMeta> User<R, G, Pu, Pr> {
    pub fn remove_group(&mut self, group: G) {
        let idx = self.groups.iter().position(|g| *g == group);

        match idx {
            Some(idx) => {
                self.groups.remove(idx);
            }
            None => {}
        }
    }

    pub fn add_group(&mut self, group: G) {
        if self.groups.contains(&group) {
        } else {
            self.groups.push(group)
        }
    }
}
