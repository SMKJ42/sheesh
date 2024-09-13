use std::{
    error::{self},
    fmt::{Debug, Display},
    time::Instant,
};

use scrypt::password_hash::{Encoding, PasswordHashString};
use serde::{Deserialize, Serialize};

use crate::{
    harness::{sqlite::SqliteDiskOpUser, DiskOp, IntoValues},
    hash::DefaultHashBrown,
};

use super::id::{DefaultIdGenerator, IdGenerator};

// Traits that provide type safety for valid inputs.
pub trait Role {}
pub trait Group {}
pub trait PublicUserMeta {}
pub trait PrivateUserMeta {}

#[derive(Clone, Debug)]
pub struct UserPublic<Pu>
where
    Pu: PublicUserMeta,
{
    public: Pu,
}

impl<Pu> IntoValues for UserPublic<Pu>
where
    Pu: PublicUserMeta + IntoValues,
{
    fn into_values(&self) -> Vec<String> {
        return self.public.into_values();
    }
}

impl<Pu> UserPublic<Pu>
where
    Pu: PublicUserMeta,
{
    pub fn new(public: Pu) -> Self {
        return Self { public };
    }

    pub fn update_public(&mut self, public: Pu) {
        self.public = public;
    }

    pub fn public(&self) -> &Pu {
        return &self.public;
    }
}

pub struct UserManagerConfig<T>
where
    T: IdGenerator,
{
    id_generator: T,
    cols: Vec<String>,
}

impl UserManagerConfig<DefaultIdGenerator> {
    pub fn new_default() -> Self {
        Self {
            id_generator: DefaultIdGenerator {},
            cols: vec![
                "username".to_string(),
                "session_id".to_string(),
                "secret".to_string(),
                "salt".to_string(),
                "id".to_string(),
                "role".to_string(),
                "groups".to_string(),
                "ban".to_string(),
            ],
        }
    }
}

impl<T> UserManagerConfig<T>
where
    T: IdGenerator + Copy,
{
    pub fn init<U: DiskOp>(&self, harness: U) -> UserManager<T, U> {
        UserManager {
            id_generator: self.id_generator,
            cols: self.cols.clone(),
            harness,
        }
    }
}

pub struct UserManager<T, V>
where
    T: IdGenerator,
    V: DiskOp,
{
    id_generator: T,
    harness: V,
    cols: Vec<String>,
}

impl UserManager<DefaultIdGenerator, SqliteDiskOpUser> {}

impl<T, V> UserManager<T, V>
where
    T: IdGenerator,
    V: DiskOp,
{
    pub fn new_user<R, G, Pu, Pr>(
        &self,
        user_name: String,
        pwd: String,
        role: R,
        public: Pu,
        private: Pr,
    ) -> Result<User<R, G, Pu, Pr>, Box<dyn error::Error>>
    where
        R: Role + Display,
        G: Group + Serialize,
        Pu: PublicUserMeta + IntoValues,
        Pr: PrivateUserMeta + IntoValues,
    {
        let id = self.id_generator.new_u64();
        let user = User::new(id, user_name, pwd, role, public, private)?;

        self.harness.insert(&user, &self.cols)?;
        return Ok(user);
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
    public: UserPublic<Pu>,
    private: Pr,
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
        pwd: String,
        role: R,
        public: Pu,
        private: Pr,
    ) -> Result<Self, Box<dyn error::Error>> {
        let start = Instant::now();

        let hash_brown = DefaultHashBrown::init();
        let salt = hash_brown.create_salt();
        let salt_gen = Instant::now().duration_since(start).as_millis();
        println!("salt_gen: {}", salt_gen);

        let secret = hash_brown.hash(pwd, &salt)?;

        println!("hash: {}", Instant::now().duration_since(start).as_millis());
        println!("secret: {}", secret);
        let secret = secret.serialize();
        let test = PasswordHashString::parse(secret.as_str(), Encoding::B64)?;

        println!("test: {}", test);

        let public = UserPublic::new(public);
        return Ok(Self {
            id,
            user_name,
            secret: secret.to_string(),
            salt: salt.to_string(),
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
    pub fn public(&self) -> &UserPublic<Pu> {
        return &self.public;
    }
    pub fn private(&self) -> &Pr {
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

    pub fn set_public_data(&mut self, public: Pu) {
        self.public.update_public(public)
    }

    pub fn set_private_data(&mut self, private: Pr) {
        self.private = private;
    }

    pub fn groups(&self) -> &Vec<G> {
        return &self.groups;
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

impl<R, G, Pu, Pr> IntoValues for User<R, G, Pu, Pr>
where
    R: Role + Display,
    G: Group + Serialize,
    Pu: PublicUserMeta + IntoValues,
    Pr: PrivateUserMeta + IntoValues,
{
    fn into_values(&self) -> Vec<String> {
        let mut values = vec![
            self.id.to_string(),
            self.user_name.clone(),
            self.secret.clone(),
            self.salt.clone(),
            self.session.unwrap_or(0).to_string(),
            self.role.to_string(),
            serde_json::to_string(&self.groups).unwrap(),
            self.ban.to_string(),
        ];

        let private = self.private.into_values();
        let public = self.public.into_values();

        values.extend(public);
        values.extend(private);

        return values;
    }
}
