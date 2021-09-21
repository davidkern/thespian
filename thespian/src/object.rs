use crate::{Type};

pub unsafe trait Object {
    fn object_type<'a>() -> &'static dyn Type where Self: Sized;
    fn get_type(&self) -> &'static dyn Type;
}

impl dyn Object {
    pub fn downcast<T: Object>(&self) -> Result<&T, ()> {
        if T::object_type() == self.get_type() {
            // SAFETY: Check confirms this is the correct type and that
            // check is correct if the unsafe Object trait is correct.
            unsafe { Ok(&*(self as *const dyn Object as *const T)) }
        } else {
            Err(())
        }
    }
}
