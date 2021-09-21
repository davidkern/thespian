use crate::{Field, Kind};

pub unsafe trait Type: Sync + Send {
    fn name(&self) -> &str;
    fn kind(&self) -> Kind;
    fn fields(&self) -> &[&dyn Field];
}

impl PartialEq for dyn Type {
    fn eq(&self, other: &Self) -> bool {
        (self as *const dyn Type) == (other as *const dyn Type)
    }
}

impl Eq for dyn Type { }

impl std::fmt::Debug for dyn Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let name = format!("&dyn {}", self.name());
        f.debug_struct(&name).finish()
    }
}
