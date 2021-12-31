mod field;
mod kind;
mod item;
mod object;
mod stage;
mod ty;

pub use field::FieldInfo;
pub use item::ItemInfo;
pub use kind::Kind;
pub use object::Object;
pub use stage::Stage;
pub use ty::Type;


// Example Case

pub trait Inc { fn inc(&mut self); }
pub trait Dec { fn dec(&mut self); }
pub trait Get { fn get(&self) -> u32; }

struct Counter {
    counter: u32,
}

impl Counter {
    fn foo() { }
}

impl Inc for Counter { fn inc(&mut self) { self.counter += 1; } }
impl Dec for Counter { fn dec(&mut self) { self.counter -= 1; } }
impl Get for Counter { fn get(&self) -> u32 { self.counter } }

// Expected Output

#[cfg(test)]
mod test {
    use super::*;

    #[allow(non_upper_case_globals)]
    const Counter: Type = {
        const FIELDS: [FieldInfo; 1] = [
            FieldInfo {
                item: ItemInfo {
                    name: "counter",
                    path: "thespian::Counter::counter"
                },
                kind: Kind::U32,
            },
        ];

        Type {
            item: ItemInfo {
                name: "Counter",
                path: "thespian::Counter",
            },
            kind: Kind::Struct {
                fields: &FIELDS,
            },
        }
    };

    unsafe impl Object for Counter {
        fn object_type() -> &'static Type {
            &Counter
        }

        fn get_type(&self) -> &'static Type {
            &Counter
        }
    }

    #[test]
    fn usage() {
        let c = Counter { counter: 0 };
        let ty = c.get_type();
    
        let obj = &c as &dyn Object;
        let ty2 = obj.get_type();
    
        assert_eq!(ty, ty2);
    }
}
