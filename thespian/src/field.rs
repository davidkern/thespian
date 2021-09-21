use crate::{Kind, Object};

pub unsafe trait Field: Sync + Send {
    fn kind(&self) -> Kind;
    fn get(&self, object: &dyn Object) -> Result<&dyn Object, ()>;
}
