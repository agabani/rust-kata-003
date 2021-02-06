use crate::domain::{CrateName, CrateVersion};

pub struct CrateMetadata {
    pub name: CrateName,
    pub version: CrateVersion,
}
