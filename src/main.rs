mod models;

extern crate colored;
extern crate dotenv;

use colored::*;
use dotenv::dotenv;
use serde::Deserialize;

use crate::models::MSM;

use axum::{routing::post, Extension, Json, Router};
use clap::Parser;
use http::Uri;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    command: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let envs: Vec<String> = env::args().collect();
    let args = Args::parse();
    match args.command.as_deref() {
        Some(v) => match v {
            "serve" => {
                println!("{}", format!("Starting Server").green());
                start_server().await;
            }
            _ => println!("Command not recognised"),
        },
        None => println!("Unkown command"),
    }
}

async fn start_server() {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    let msm = MSM::new();

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
    let services = msm.services;
    let service = services.get(service_name_from_uri).unwrap().clone();

    let exec_result = service.execute();
    format!("{:?}", exec_result.to_string())
}
