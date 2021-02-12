use crate::domain::{CrateDependency, CrateName, CrateVersion};

#[derive(Debug, PartialEq)]
pub struct CrateMetadata {
    pub name: CrateName,
    pub version: CrateVersion,
    pub dependencies: Vec<CrateDependency>,
}
