mod db;
mod models;
mod utils;

extern crate dotenv;

use dotenv::dotenv;
use serde::Deserialize;

use crate::models::MSM;
use std::env;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use http::Uri;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // for (key, value) in env::vars() {
    //     println!("ENV: {}: {}", key, value);
    // }

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    let msm = MSM::new();
    // let services = msm.services;
    // for service in services {
    //     service.execute();
    // }

    Router::new()
        .route("/:id", post(execute))
        .layer(Extension(msm))
}

#[derive(Deserialize)]
struct Payload {
    data: String,
}

async fn execute(uri: Uri, Json(payload): Json<Payload>, Extension(msm): Extension<MSM>) -> String {
    // println!("{}", uri);
    let uri_string = uri.to_string();
    let service_name_vec: Vec<&str> = uri_string.split("/").collect();
    let service_name_from_uri = service_name_vec[1];
    println!("{}", service_name_from_uri);
    for service in msm.services {
        if service.name == service_name_from_uri && service.availible {
            return format!("{:?}", service.execute());
        }
    }
    format!("{:?}", "hello")
}
