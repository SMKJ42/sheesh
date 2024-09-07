use rusqlite::{Connection, Result};

use super::DiskOp;

pub struct SqliteDiskOpUser {
    connection: Connection,
}

// impl DiskOpUser for SqliteDiskOpUser {
//     fn ban(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn update_public(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn insert_group(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn remove_group(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn update_private(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
//     fn write_role(&self) -> Result<(), Box<dyn std::error::Error>> {
//         unimplemented!()
//     }
// }

impl SqliteDiskOpUser {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

impl DiskOp for SqliteDiskOpUser {
    fn delete<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn insert<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn read<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

pub struct SqliteDiskOpSession {
    connection: Connection,
}

impl SqliteDiskOpSession {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

impl DiskOp for SqliteDiskOpSession {
    fn delete<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn insert<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn read<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

pub struct SqliteDiskOpToken {
    connection: Connection,
}

impl SqliteDiskOpToken {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

impl DiskOp for SqliteDiskOpToken {
    fn delete<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn insert<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn read<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update<User>(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
