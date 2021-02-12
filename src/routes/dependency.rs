use crate::crates_io_client::CratesIoClient;
use crate::domain::{CrateName, CrateVersion};
use crate::postgres_client::PostgresClient;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Query {
    #[serde(rename = "name")]
    pub crate_name: String,
    #[serde(rename = "version")]
    pub crate_version: String,
}

#[derive(Serialize)]
pub struct Response {
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
    #[serde(rename = "requirement")]
    pub requirement: String,
}

#[tracing::instrument(
    skip(crates_io_client, postgres_client, query),
    fields(
        crate_name = %query.crate_name,
        crate_version = %query.crate_version,
    ),
)]
pub async fn dependency_query(
    query: web::Query<Query>,
    crates_io_client: web::Data<CratesIoClient>,
    postgres_client: web::Data<PostgresClient>,
) -> Result<HttpResponse, HttpResponse> {
    let name = CrateName::parse(&query.crate_name)?;
    let version = CrateVersion::parse(&query.crate_version)?;

    let metadata = if let Some(metadata) = postgres_client
        .get_crate_metadata(&name, &version)
        .await
        .unwrap()
    {
        metadata
    } else {
        let metadata = crates_io_client
            .get_ref()
            .dependencies(&name, &version)
            .await
            .ok_or_else(|| HttpResponse::NotFound().finish())?;

        postgres_client
            .save_crate_metadata(&metadata)
            .await
            .unwrap();

        metadata
    };

    let json = Response {
        data: vec![Node {
            name: metadata.name.as_str().to_owned(),
            version: metadata.version.as_str().to_owned(),
            edges: metadata
                .dependencies
                .iter()
                .map(|dependency| Edge {
                    relationship: format!("dependency.{}", dependency.type_.as_str()),
                    node: RelatedNode {
                        name: dependency.name.as_str().to_owned(),
                        requirement: dependency.requirement.as_str().to_owned(),
                    },
                })
                .collect(),
        }],
    };

    Ok(HttpResponse::Ok().json(&json))
}
