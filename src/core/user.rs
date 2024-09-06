use crate::db::{sqlite::SqliteDiskOpUser, DiskOp};

use super::id::IdGenerator;

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
    user_name: String,
    public: Pu,
    id: u64,
}

impl<Pu> UserPublic<Pu>
where
    Pu: PublicUserMeta,
{
    pub fn new(id: u64, user_name: String, public: Pu) -> Self {
        return Self {
            user_name,
            public,
            id,
        };
    }

    pub fn new_with_id(user_name: String, public: Pu, id: u64) -> Self {
        return Self {
            user_name,
            public,
            id,
        };
    }

    pub fn update_public(&mut self, public: Pu) {
        self.public = public;
    }

    pub fn public(&self) -> &Pu {
        return &self.public;
    }

    pub fn update_user_name(&mut self, user_name: String) {
        self.user_name = user_name;
    }

    pub fn user_name(&self) -> &String {
        return &self.user_name;
    }

    pub fn id(&self) -> u64 {
        return self.id;
    }
}

pub struct UserManager<T, V>
where
    T: IdGenerator,
    V: DiskOp,
{
    id_generator: T,
    db_harness: V,
}

impl<T> UserManager<T, SqliteDiskOpUser>
where
    T: IdGenerator,
{
    pub fn init(id_generator: T) -> Self {
        Self {
            id_generator,
            db_harness: SqliteDiskOpUser {},
        }
    }
}

impl<T, V> UserManager<T, V>
where
    T: IdGenerator,
    V: DiskOp,
{
    pub fn new_user<R, G, Pu, Pr>(
        &self,
        user_name: String,
        role: R,
        public: Pu,
        private: Pr,
    ) -> User<R, G, Pu, Pr>
    where
        R: Role,
        G: Group,
        Pu: PublicUserMeta,
        Pr: PrivateUserMeta,
    {
        let id = self.id_generator.new_u64();
        return User::new(id, user_name, role, public, private);
    }
}

pub struct User<R, G, Pu, Pr>
where
    R: Role,
    G: Group,
    Pu: PublicUserMeta,
    Pr: PrivateUserMeta,
{
    role: R,
    groups: Vec<G>,
    public: UserPublic<Pu>,
    private: Pr,
    banned: bool,
}

impl<R, G, Pu, Pr> User<R, G, Pu, Pr>
where
    R: Role,
    G: Group,
    Pu: PublicUserMeta,
    Pr: PrivateUserMeta,
{
    pub fn new(id: u64, user_name: String, role: R, public: Pu, private: Pr) -> Self {
        let public = UserPublic::new(id, user_name, public);
        Self {
            role,
            groups: Vec::new(),
            public,
            private,
            banned: false,
        }
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
        self.banned = true;
    }

    pub fn unban(&mut self) {
        self.banned = false;
    }

    pub fn is_banned(&self) -> bool {
        return self.banned;
    }

    pub fn set_public_data(&mut self, public: Pu) {
        self.public.update_public(public)
    }

    pub fn set_private_data(&mut self, private: Pr) {
        self.private = private;
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
