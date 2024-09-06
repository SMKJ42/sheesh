pub trait HashGenerator {}

#[derive(Clone, Copy)]
pub struct DefaultHashGenerator {}

impl HashGenerator for DefaultHashGenerator {}

impl DefaultHashGenerator {
    pub fn init() -> Self {
        unimplemented!();
    }
}
