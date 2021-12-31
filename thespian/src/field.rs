use crate::{ItemInfo, Kind, Object};

#[derive(Debug)]
pub struct FieldInfo {
    pub item: ItemInfo,
    pub kind: Kind,
}

impl FieldInfo {
    fn get(&self, object: &dyn Object) -> Result<&dyn Object, ()> {
        Err(())
    }
}
