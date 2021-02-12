use crate::telemetry::TraceErrorExt;
use reqwest::StatusCode;

mod dependencies;

pub struct CratesIoClient {
    base_address: String,
    client: reqwest::Client,
}

impl CratesIoClient {
    pub fn new(base_address: &str, user_agent: &str) -> Result<CratesIoClient, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            user_agent.parse().expect("Failed to parse user agent."),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            base_address: base_address.to_owned(),
            client,
        })
    }

    #[tracing::instrument(skip(self))]
    async fn get<T: for<'de> serde::Deserialize<'de>>(&self, path: &str) -> Option<T> {
        tracing::info!("fetching data");
        let url = format!("{}{}", self.base_address, path);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .trace_err()
            .expect("Failed to send request.");

        match response.status() {
            StatusCode::OK => {}
            StatusCode::NOT_FOUND => return None,
            status_code => panic!("Unsupported response: {}", status_code),
        }

        let data = response
            .json()
            .await
            .trace_err()
            .expect("Failed to read response.");

        Some(data)
    }
}
