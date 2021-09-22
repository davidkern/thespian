use super::{AssociatedConstant, AssociatedFunction, AssociatedType};

pub trait Trait {
    fn constants(&self) -> &[&dyn AssociatedConstant];
    fn functions(&self) -> &[&dyn AssociatedFunction];
    fn types(&self) -> &[&dyn AssociatedType];
}
