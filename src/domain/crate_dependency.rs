use crate::domain::{CrateDependencyType, CrateName, CrateRequirement};

#[derive(Debug, PartialEq)]
pub struct CrateDependency {
    pub name: CrateName,
    pub requirement: CrateRequirement,
    pub type_: CrateDependencyType,
}
