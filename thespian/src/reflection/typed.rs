use super::Type;

/// Provides a static reference to the Thespian type value for a rust type
/// which implements this trait.
/// 
/// This trait is unsafe to implement since returning an incorrect type can
/// cause UB during Invocation.
pub unsafe trait Typed {
    fn typed() -> &'static dyn Type;
}
