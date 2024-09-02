pub trait HashGenerator {}

pub struct DefaultHashGenerator {}

impl HashGenerator for DefaultHashGenerator {}

impl DefaultHashGenerator {
    pub fn init() -> Self {
        unimplemented!();
    }
}
