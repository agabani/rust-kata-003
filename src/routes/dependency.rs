use actix_web::{web, HttpResponse};

pub mod view_models {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct Query {
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "version")]
        pub version: String,
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

pub async fn dependency_query(web::Query(query): web::Query<view_models::Query>) -> HttpResponse {
    HttpResponse::Ok().json(view_models::Result {
        data: vec![view_models::Node {
            name: query.name,
            version: query.version,
            edges: vec![],
        }],
    })
}
