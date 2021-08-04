#[derive(Debug)]
pub struct ItemInfo {
    pub name: &'static str,
    pub path: &'static str,
}

impl PartialEq for ItemInfo {
    fn eq(&self, other: &Self) -> bool {
        // fast path: compare pointers
        *self.path == *other.path

        // slow path: compare strings
        || self.path == other.path
    }
}
