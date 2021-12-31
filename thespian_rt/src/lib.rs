// #[derive(Clone, Eq, PartialEq, Debug)]
// pub enum Reflection {
//     Error,
//     Capability { name: String },
// }

// unsafe trait Reflect {
//     fn reflect(&self) -> Reflection;
//     fn invoke(&self, reflection: Reflection) -> Reflection;
// }

// #[cfg(test)]
// mod test {
//     mod subject {
//         use super::super::{Reflection, Reflect};

//         pub trait TUnit {}
//         unsafe impl Reflect for &dyn TUnit {
//             fn reflect(&self) -> Reflection {
//                 Reflection::Capability {
//                     name: "TUnit".to_string(),
//                 }
//             }

//             fn invoke(&self, reflection: Reflection) -> Reflection {
//                 Reflection::Error
//             }
//         }

//         pub trait TUnit2 {}
//         unsafe impl Reflect for &dyn TUnit2 {
//             fn reflect(&self) -> Reflection {
//                 Reflection::Capability {
//                     name: "TUnit2".to_string(),
//                 }
//             }

//             fn invoke(&self, reflection: Reflection) -> Reflection {
//                 Reflection::Error
//             }
//         }

//         // impl<T> Reflect<*const dyn TUnit> for T where T: TUnit {
//         //     fn reflect(item: *const dyn TUnit) -> Reflection {
//         //         Reflection {
//         //             name: "TUnit".to_string(),
//         //         }
//         //     }
//         // }

//         // pub trait TUnit2 {}
//         // impl<T> Reflect<*const dyn TUnit2> for T where T: TUnit2 {
//         //     fn reflect(item: *const dyn TUnit2) -> Reflection {
//         //         Reflection {
//         //             name: "TUnit2".to_string(),
//         //         }
//         //     }
//         // }

//         pub trait T {
//             fn x(&self) -> u8;
//             fn y(&self) -> u16;
//             fn set_x(&mut self, value: u8);
//             fn set_y(&mut self, value: u16);
//         }

//         pub struct S {
//             x: u8,
//             y: u16,
//         }

//         impl T for S {
//             fn x(&self) -> u8 { self.x }
//             fn y(&self) -> u16 { self.y }
//             fn set_x(&mut self, value: u8) {
//                 self.x = value;
//             }
//             fn set_y(&mut self, value: u16) {
//                 self.y = value;
//             }
//         }
//     }

//     #[test]
//     fn usage() {
//         // Goal: Export the definition of a trait and allow the methods of that trait to be called remotely.
//         // Need a map from trait, fn -> TraitInfo, MethodInfo
//         // TraitInfo.invoke(obj)

//         use super::{Reflection, Reflect};
//         use subject::{TUnit, TUnit2};

//         struct Test;
//         impl TUnit for Test {}
//         impl TUnit2 for Test {}

//         let test = Test{};

//         // TUnit
//         assert_eq!(
//             (&test as &dyn TUnit).reflect(),
//             Reflection::Capability {
//                 name: "TUnit".to_string(),
//             }
//         );

//         // TUnit2
//         assert_eq!(
//             (&test as &dyn TUnit2).reflect(),
//             Reflection::Capability {
//                 name: "TUnit2".to_string(),
//             }
//         );
//     }

//     #[test]
//     fn restriction() {
//         // Given a capability, derive a new capablity with restricted authority
        
//         trait Read { fn read(&self); }

//         trait Write { fn write(&self); }

//         trait ReadWrite: Read + Write {
//             fn into_read(&self) -> &dyn Read;
//             fn into_write(&self) -> &dyn Write;
//         }

//         impl<T: Read + Write> ReadWrite for T {
//             fn into_read(&self) -> &dyn Read { self }
//             fn into_write(&self) -> &dyn Write { self }
//         }

//         struct O;

//         impl Read for O { fn read(&self) { } }

//         impl Write for O { fn write(&self) { } }

//         let obj = O;
//         let rw = &obj as &dyn ReadWrite;

//         let r = rw.into_read();
//         let w = rw.into_write();

//         w.write();
//     }
// }
