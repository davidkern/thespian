use super::Type;
use std::marker::PhantomData;

/// Provides a static reference to the Thespian type value for a rust type
/// which implements this trait.
/// 
/// This trait is unsafe to implement since returning an incorrect type can
/// cause UB during Invocation.
pub unsafe trait Typed {
    fn typed(&self) -> &'static dyn Type;
}

/// A wrapper type around T which allows specialization for the Typed trait
/// via autoref functionality:
/// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization.
pub struct TypeWrapper<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> TypeWrapper<T> {
    pub fn new() -> Self {
        TypeWrapper(PhantomData)
    }
}

impl<T: ?Sized> From<&T> for TypeWrapper<T> {
    fn from(_: &T) -> Self {
        TypeWrapper(PhantomData)
    }
}

/// Represents the type for all types which do not implement a specialiation
/// of the Typed trait.
struct UnknownType;
static UNKNOWN: UnknownType = UnknownType;

unsafe impl Type for UnknownType {
    fn identifier(&self) -> &str {
        "<unknown>"
    }
}

/// The default implementation of Typed returns the Type of the UnknownType
unsafe impl<T: ?Sized> Typed for &TypeWrapper<T> {
    fn typed(&self) -> &'static dyn Type {
        &UNKNOWN
    }
}

/// Get the Type trait object for a type.
#[macro_export]
macro_rules! typed {
    ($t:ty) => {
        {
            use crate::reflection::{Typed, TypeWrapper};
            (&TypeWrapper::<$t>::new()).typed()
        }
    };
}

/// Get the Type trait object for a value.
#[macro_export]
macro_rules! ty {
    ($e:expr) => {
        {
            use crate::reflection::{Typed, TypeWrapper};
            (&TypeWrapper::from(&$e)).typed()
        }
    };
}
