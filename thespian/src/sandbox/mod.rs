mod field;
mod kind;
mod object;
mod stage;
mod ty;

pub use field::Field;
pub use kind::Kind;
pub use object::Object;
pub use stage::Stage;
pub use ty::Type;


// Example Case

pub trait Inc { fn inc(&mut self); }
pub trait Dec { fn dec(&mut self); }
pub trait Get { fn get(&self) -> u32; }

pub struct Counter {
    counter: u32,
}

impl Counter {
    pub fn foo() { }
}

impl Inc for Counter { fn inc(&mut self) { self.counter += 1; } }
impl Dec for Counter { fn dec(&mut self) { self.counter -= 1; } }
impl Get for Counter { fn get(&self) -> u32 { self.counter } }

// Expected Output

#[cfg(test)]
mod test {
    use super::*;

    #[allow(non_upper_case_globals)]
    static Counter: &'static dyn Type = {
        struct Meta;
        static META: Meta = Meta;

        struct Item1;
        static ITEM1: Item1 = Item1;

        unsafe impl Field for Item1 {
            fn kind(&self) -> Kind {
                Kind::U32
            }

            fn get(&self, _object: &dyn Object) -> Result<&dyn Object, ()> {
                Err(())
            }
        }

        static ITEMS: [&dyn Field; 1] = [
            &ITEM1,
        ];

        unsafe impl Type for Meta {
            fn name(&self) -> &str {
                "Counter"
            }

            fn kind(&self) -> Kind {
                Kind::Struct
            }

            fn fields(&self) -> &[&dyn Field] {
                &ITEMS[..]
            }
        }

        &META
    };

    unsafe impl Object for Counter {
        fn object_type() -> &'static dyn Type {
            Counter
        }

        fn get_type(&self) -> &'static dyn Type {
            Counter
        }
    }

    #[test]
    fn usage() {
        let c = Counter { counter: 0 };
        let ty = c.get_type();
    
        let obj = &c as &dyn Object;
        let ty2 = obj.get_type();
    
        assert_eq!(ty, ty2);

        struct Other;
        unsafe impl Type for Other {
            fn name(&self) -> &str { "Other" }
            fn kind(&self) -> Kind { Kind::Struct }
            fn fields(&self) -> &[&dyn Field] {
                &[]
            }
        }
        unsafe impl Object for Other {
            fn object_type() -> &'static dyn Type {
                &Other
            }
    
            fn get_type(&self) -> &'static dyn Type {
                &Other
            }
        }
    
        let o = Other;
        let oty = o.get_type();

        assert_ne!(ty, oty);
    }
}
