use crate::domain::{CrateMetadata, CrateName, CrateVersion};

pub struct CratesIoClient;

impl CratesIoClient {
    pub async fn query(&self, name: CrateName, version: CrateVersion) -> CrateMetadata {
        CrateMetadata { name, version }
    }
}
