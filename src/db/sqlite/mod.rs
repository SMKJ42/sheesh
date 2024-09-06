use super::{DiskOp, DiskOpUser};

pub struct SqliteDiskOpUser {}

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

impl DiskOp for SqliteDiskOpUser {
    fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn insert(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn read(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

pub struct SqliteDiskOpSession {}

impl DiskOp for SqliteDiskOpSession {
    fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn insert(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn read(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

pub struct SqliteDiskOpToken {}

impl DiskOp for SqliteDiskOpToken {
    fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn insert(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn read(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
    fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
