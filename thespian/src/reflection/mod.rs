mod associated_constant;
mod associated_function;
mod associated_type;
mod r#trait;
mod r#type;
mod typed;
pub mod typing;

pub use associated_constant::AssociatedConstant;
pub use associated_function::AssociatedFunction;
pub use associated_type::AssociatedType;
pub use r#trait::Trait;
pub use r#type::Type;
pub use typed::{Typed, TypeWrapper};

#[cfg(test)]
mod test {
    #[test]
    fn bounded_introspection() {

    }
    
    #[test]
    fn introspection() {
        struct Type {
            count: u8,
        }

        trait Introspect {
            fn introspect(&self) -> Type;
        }

        impl Introspect for u8 {
            fn introspect(&self) -> Type {
                Type {
                    count: 0,
                }
            }
        }

        impl <T: Introspect> Introspect for &T {
            fn introspect(&self) -> Type {
                Type {
                    count: (*self).introspect().count + 1,
                }
            }
        }

        // <u8 as Introspect>::introspect();
        // <&u8 as Introspect>::introspect();
        // <&&u8 as Introspect>::introspect();
        // <*const u8 as Introspect>::introspect();
        // <*mut u8 as Introspect>::introspect();
        // <*mut *const u8 as Introspect>::introspect();

        let x: u8 = 0;
        assert_eq!(x.introspect().count, 0);
        let rx = &x;
        assert_eq!(rx.introspect().count, 0);
        let rrx = &rx;
        assert_eq!(rrx.introspect().count, 1);
    }

    #[test]
    fn reflector() {
        use std::cell::RefCell;
        use slab::Slab;

        struct TypeInfo;

        struct Reflector {
            type_info: RefCell<Slab<TypeInfo>>,
        }

        impl Reflector {
            pub fn reflect<'r, T>(&'r self, _object: &T) -> Type<'r> {
                let key = self.type_info.borrow_mut().insert(TypeInfo{});

                Type {
                    reflector: self,
                    key,
                }
            }
        }

        struct Type<'r> {
            reflector: &'r Reflector,
            key: usize,
        }
    }

    #[test]
    fn strip_references() {
        #[derive(Debug, PartialEq, Eq)]
        enum ReferenceType {
            Ref,
            RefMut,
            Value,
        }

        trait Reference {
            fn reference(&self) -> ReferenceType;
        }

        struct X;

        impl Reference for X { fn reference(&self) -> ReferenceType { ReferenceType::Value } }
        impl Reference for &X { fn reference(&self) -> ReferenceType { ReferenceType::Ref } }
        impl Reference for &mut X { fn reference(&self) -> ReferenceType { ReferenceType::RefMut } }

        let mut x = X;

        assert_eq!((&x).reference(), ReferenceType::Value);
        assert_eq!((&&x).reference(), ReferenceType::Ref);
        assert_eq!((&&mut x).reference(), ReferenceType::RefMut);
    }
}
