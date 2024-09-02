use scrypt::Scrypt;

pub enum PasswordMethod {
    Scrypt(Scrypt),
}

impl PasswordMethod {
    pub fn scrypt() -> Self {
        return Self::Scrypt(Scrypt);
    }
}

impl PasswordMethod {}
