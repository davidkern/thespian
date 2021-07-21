#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Reflection {
    name: String,
}

trait Reflect<T> {
    fn reflect(item: T) -> Reflection;
}

#[cfg(test)]
mod test {
    mod subject {
        use super::super::{Reflection, Reflect};

        pub trait TUnit {}
        impl<T> Reflect<*const dyn TUnit> for T where T: TUnit {
            fn reflect(item: *const dyn TUnit) -> Reflection {
                Reflection {
                    name: "TUnit".to_string(),
                }
            }
        }

        pub trait TUnit2 {}
        impl<T> Reflect<*const dyn TUnit2> for T where T: TUnit2 {
            fn reflect(item: *const dyn TUnit2) -> Reflection {
                Reflection {
                    name: "TUnit2".to_string(),
                }
            }
        }

        pub trait T {
            fn x(&self) -> u8;
            fn y(&self) -> u16;
            fn set_x(&mut self, value: u8);
            fn set_y(&mut self, value: u16);
        }

        pub struct S {
            x: u8,
            y: u16,
        }

        impl T for S {
            fn x(&self) -> u8 { self.x }
            fn y(&self) -> u16 { self.y }
            fn set_x(&mut self, value: u8) {
                self.x = value;
            }
            fn set_y(&mut self, value: u16) {
                self.y = value;
            }
        }
    }

    #[test]
    fn usage() {
        // Goal: Export the definition of a trait and allow the methods of that trait to be called remotely.
        // Need a map from trait, fn -> TraitInfo, MethodInfo
        // TraitInfo.invoke(obj)

        use super::{Reflection, Reflect};
        use subject::{TUnit, TUnit2};

        struct Test;
        impl TUnit for Test {}
        impl TUnit2 for Test {}

        let test = Test{};

        // TUnit
        assert_eq!(
            Test::reflect(&test as *const dyn TUnit),
            Reflection {
                name: "TUnit".to_string(),
            }
        );

        // TUnit2
        assert_eq!(
            Test::reflect(&test as *const dyn TUnit2),
            Reflection {
                name: "TUnit2".to_string(),
            }
        );
    }
}
