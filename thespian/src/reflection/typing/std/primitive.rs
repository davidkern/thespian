macro_rules! impl_typed {
    ($identifier:ident) => {
        pub mod $identifier {
            use crate::reflection::{Type, Typed, TypeWrapper};
        
            struct Ty;
            static TY: Ty = Ty;
            
            unsafe impl Type for Ty {
                fn identifier(&self) -> &str {
                    stringify!($identifier)
                }
            }
            
            unsafe impl Typed for TypeWrapper<$identifier> {
                fn typed(&self) -> &'static dyn Type {
                    &TY
                }
            }    
        }
    }
}

impl_typed!(bool);
impl_typed!(char);
impl_typed!(f32);
impl_typed!(f64);
impl_typed!(i8);
impl_typed!(i16);
impl_typed!(i32);
impl_typed!(i64);
impl_typed!(i128);
impl_typed!(isize);
impl_typed!(str);
impl_typed!(u8);
impl_typed!(u16);
impl_typed!(u32);
impl_typed!(u64);
impl_typed!(u128);
impl_typed!(usize);

#[cfg(test)]
mod test {
    use crate::{typed, ty};

    #[test]
    fn typed_primitives() {
        assert_eq!(typed!(bool).identifier(), "bool");
        assert_eq!(typed!(f32).identifier(), "f32");
        assert_eq!(typed!(f64).identifier(), "f64");
        assert_eq!(typed!(i8).identifier(), "i8");
        assert_eq!(typed!(i16).identifier(), "i16");
        assert_eq!(typed!(i32).identifier(), "i32");
        assert_eq!(typed!(i64).identifier(), "i64");
        assert_eq!(typed!(i128).identifier(), "i128");
        assert_eq!(typed!(isize).identifier(), "isize");
        assert_eq!(typed!(str).identifier(), "str");
        assert_eq!(typed!(u8).identifier(), "u8");
        assert_eq!(typed!(u16).identifier(), "u16");
        assert_eq!(typed!(u32).identifier(), "u32");
        assert_eq!(typed!(u64).identifier(), "u64");
        assert_eq!(typed!(u128).identifier(), "u128");
        assert_eq!(typed!(usize).identifier(), "usize");
    }

    #[test]
    fn typed_unknown() {
        struct Untyped;

        assert_eq!(typed!(Untyped).identifier(), "<unknown>");
    }

    #[test]
    fn typed_primitive_values() {
        let v_bool: bool = Default::default();
        let v_f32: f32 = Default::default();
        let v_f64: f64 = Default::default();
        let v_i8: i8 = Default::default();
        let v_i16: i16 = Default::default();
        let v_i32: i32 = Default::default();
        let v_i64: i64 = Default::default();
        let v_i128: i128 = Default::default();
        let v_isize: isize = Default::default();
        let v_str: &str = Default::default();
        let v_u8: u8 = Default::default();
        let v_u16: u16 = Default::default();
        let v_u32: u32 = Default::default();
        let v_u64: u64 = Default::default();
        let v_u128: u128 = Default::default();
        let v_usize: usize = Default::default();

        assert_eq!(ty!(v_bool).identifier(), "bool");
        assert_eq!(ty!(v_f32).identifier(), "f32");
        assert_eq!(ty!(v_f64).identifier(), "f64");
        assert_eq!(ty!(v_i8).identifier(), "i8");
        assert_eq!(ty!(v_i16).identifier(), "i16");
        assert_eq!(ty!(v_i32).identifier(), "i32");
        assert_eq!(ty!(v_i64).identifier(), "i64");
        assert_eq!(ty!(v_i128).identifier(), "i128");
        assert_eq!(ty!(v_isize).identifier(), "isize");
        assert_eq!(ty!(*v_str).identifier(), "str");
        assert_eq!(ty!(v_u8).identifier(), "u8");
        assert_eq!(ty!(v_u16).identifier(), "u16");
        assert_eq!(ty!(v_u32).identifier(), "u32");
        assert_eq!(ty!(v_u64).identifier(), "u64");
        assert_eq!(ty!(v_u128).identifier(), "u128");
        assert_eq!(ty!(v_usize).identifier(), "usize");
    }

    #[test]
    fn typed_unknown_value() {
        struct Untyped;
        let untyped: Untyped = Untyped;

        assert_eq!(ty!(untyped).identifier(), "<unknown>");
    }
}
