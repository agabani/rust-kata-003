use crate::domain::{CrateName, CrateRelationship, CrateVersion};

pub struct CrateMetadata {
    pub name: CrateName,
    pub version: CrateVersion,
    pub relationship: CrateRelationship,
}
