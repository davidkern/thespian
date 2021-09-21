use crate::{Type, Object};

pub trait Stage {
    fn enter(&mut self, ty: &dyn Type) -> Result<&dyn Object, ()>;
}
