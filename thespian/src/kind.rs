pub use crate::FieldInfo;

#[derive(Debug)]
pub enum Kind {
    U32,
    Struct {
        fields: &'static [FieldInfo]
    },
}
