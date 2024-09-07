mod hashbrown;
pub mod method;
pub mod rng;

pub mod error;
pub use hashbrown::*;

pub trait HashGenerator {}

#[derive(Clone, Copy)]
pub struct DefaultHashGenerator {}

impl HashGenerator for DefaultHashGenerator {}

impl DefaultHashGenerator {
    pub fn init() -> Self {
        unimplemented!();
    }
}
