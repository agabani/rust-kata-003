use crate::crates_io_client::CratesIoClient;
use crate::domain::{CrateName, CrateVersion};
use actix_web::{web, HttpResponse};

pub mod view_models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct Query {
        #[serde(rename = "name")]
        pub crate_name: String,
        #[serde(rename = "version")]
        pub crate_version: String,
    }

    #[derive(Serialize)]
    pub struct Result {
        #[serde(rename = "data")]
        pub data: Vec<Node>,
    }

    #[derive(Serialize)]
    pub struct Node {
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "version")]
        pub version: String,
        #[serde(rename = "edges")]
        pub edges: Vec<Edge>,
    }

    #[derive(Serialize)]
    pub struct Edge {
        #[serde(rename = "relationship")]
        pub relationship: String,
        #[serde(rename = "node")]
        pub node: RelatedNode,
    }

    #[derive(Serialize)]
    pub struct RelatedNode {
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "version")]
        pub version: String,
    }
}

#[tracing::instrument(
    skip(client, query),
    fields(
        crate_name = %query.crate_name,
        crate_version = %query.crate_version,
    ),
)]
pub async fn dependency_query(
    web::Query(query): web::Query<view_models::Query>,
    client: web::Data<CratesIoClient>,
) -> Result<HttpResponse, HttpResponse> {
    let name = CrateName::parse(&query.crate_name)?;
    let version = CrateVersion::parse(&query.crate_version)?;

    let metadata = client.get_ref().query(name, version).await;

    Ok(HttpResponse::Ok().json(view_models::Result {
        data: vec![view_models::Node {
            name: metadata.name.as_str().to_owned(),
            version: metadata.version.as_str().to_owned(),
            edges: vec![],
        }],
    }))
}
