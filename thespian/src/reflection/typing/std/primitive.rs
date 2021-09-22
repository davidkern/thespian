use crate::reflection::{Type, Typed};

struct Bool;
static TYPE_BOOL: Bool = Bool;

unsafe impl Type for Bool { }

unsafe impl Typed for bool {
    fn typed() -> &'static dyn Type {
        &TYPE_BOOL
    }
}