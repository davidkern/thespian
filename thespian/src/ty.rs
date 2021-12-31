use crate::{FieldInfo, ItemInfo, Kind};

#[derive(Debug)]
pub struct Type {
    pub item: ItemInfo,
    pub kind: Kind,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}
