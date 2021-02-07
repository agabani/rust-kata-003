use crate::crates_io_client::CratesIoClient;
use reqwest::Error;

#[derive(serde::Deserialize)]
pub struct CratesIoConfiguration {
    pub base_address: String,
    pub user_agent: String,
}

impl CratesIoConfiguration {
    pub fn client(&self) -> Result<CratesIoClient, Error> {
        CratesIoClient::new(&self.base_address, &self.user_agent)
    }
}
