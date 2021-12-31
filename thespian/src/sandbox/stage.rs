use crate::sandbox::{Type, Object};

pub trait Stage {
    fn enter(&mut self, ty: &dyn Type) -> Result<&dyn Object, ()>;
}
