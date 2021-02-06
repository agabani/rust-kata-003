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
    name = "Querying dependency",
    skip(query),
    fields(
        crate_name = %query.crate_name,
        crate_version = %query.crate_version,
    ),
)]
pub fn dependency_query(web::Query(query): web::Query<view_models::Query>) -> HttpResponse {
    HttpResponse::Ok().json(view_models::Result {
        data: vec![view_models::Node {
            name: query.crate_name,
            version: query.crate_version,
            edges: vec![],
        }],
    })
}
