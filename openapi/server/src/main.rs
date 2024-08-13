use std::{collections::HashMap, fs, io::BufWriter};

// axum example
use axum::{
    extract::{Path, Query},
    routing::{self, get, post},
    Json,
};
use http::header::CONTENT_TYPE;
use oasgen::{oasgen, OaSchema, PathMethod, Server};
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::{self, CorsLayer},
    trace::TraceLayer,
};

#[derive(OaSchema, Deserialize, Serialize, Debug)]
pub struct SendCode {
    pub mobile: String,
}

#[derive(Serialize, OaSchema, Debug)]
pub struct SendCodeResponse {
    pub found_account: bool,
}

#[oasgen]
async fn send_code(_body: Json<SendCode>) -> Json<SendCodeResponse> {
    Json(SendCodeResponse {
        found_account: false,
    })
}

#[oasgen]
async fn send_code2(
    Path(path1): Path<String>,
    // query work in swagger ui, but not with ts generator
    // Query(query1): Query<SendCode>,
    Json(mut data): Json<SendCode>,
) -> Json<SendCode> {
    // Json(format!("path1: {path1}\ndata: {data:?}"))
    data.mobile += &path1;
    Json(data)
}

#[tokio::main]
async fn main() {
    let server = Server::axum()
        .post("/send-code", send_code)
        .post("/send-code2/:some_path_what", send_code2)
        .inspect(|openapi| {
            let file = fs::File::create("openapi.yaml").expect("create file fail");
            serde_yaml::to_writer(BufWriter::new(file), openapi).expect("write openapi fail");
        })
        // .write_and_exit_if_env_var_set("openapi.yaml")
        .route_yaml_spec("/openapi.yaml") // the spec will be available at /openapi.yaml
        .route_json_spec("/openapi.json") // the spec will be available at /openapi.json
        .swagger_ui("/ui/") // the swagger UI will be available at /openapi/.
        .freeze();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let router = axum::Router::new()
        .route("/healthcheck", get(|| async { "OK" }))
        .route("/send-code1/:some_path", post(send_code2))
        .merge(server.into_router())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_origin(cors::Any)
                .allow_headers([CONTENT_TYPE]),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
