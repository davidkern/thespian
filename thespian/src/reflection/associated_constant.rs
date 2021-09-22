pub use super::Type;

pub trait AssociatedConstant {
    fn identifier(&self) -> &str;
    fn typed(&self) -> &dyn Type;
}
