macro_rules! typed {
    ($identifier:ident) => {
        pub mod $identifier {
            use crate::reflection::{Type, Typed};
        
            struct Ty;
            static TY: Ty = Ty;
            
            unsafe impl Type for Ty {
                fn identifier(&self) -> &str {
                    stringify!($identifier)
                }
            }
            
            unsafe impl Typed for $identifier {
                fn typed() -> &'static dyn Type {
                    &TY
                }
            }    
        }        
    }
}

typed!(bool);
typed!(char);
typed!(f32);
typed!(f64);
typed!(i8);
typed!(i16);
typed!(i32);
typed!(i64);
typed!(i128);
typed!(isize);
typed!(str);
typed!(u8);
typed!(u16);
typed!(u32);
typed!(u64);
typed!(u128);
typed!(usize);

#[cfg(test)]
mod test {
    use crate::reflection::Typed;

    #[test]
    fn typed_primitives() {
        assert_eq!(bool::typed().identifier(), "bool");
        assert_eq!(f32::typed().identifier(), "f32");
        assert_eq!(f64::typed().identifier(), "f64");
        assert_eq!(i8::typed().identifier(), "i8");
        assert_eq!(i16::typed().identifier(), "i16");
        assert_eq!(i32::typed().identifier(), "i32");
        assert_eq!(i64::typed().identifier(), "i64");
        assert_eq!(i128::typed().identifier(), "i128");
        assert_eq!(isize::typed().identifier(), "isize");
        assert_eq!(str::typed().identifier(), "str");
        assert_eq!(u8::typed().identifier(), "u8");
        assert_eq!(u16::typed().identifier(), "u16");
        assert_eq!(u32::typed().identifier(), "u32");
        assert_eq!(u64::typed().identifier(), "u64");
        assert_eq!(u128::typed().identifier(), "u128");
        assert_eq!(usize::typed().identifier(), "usize");
    }
}