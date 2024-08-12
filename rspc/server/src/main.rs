use std::path::PathBuf;

use axum::routing::get;
use rspc::Router;
use tower_http::cors::CorsLayer;

fn router() -> Router<()> {
    <Router>::new()
        .query("version", |t| t(|ctx, input: ()| env!("CARGO_PKG_VERSION")))
        .build()
}
mod t1 {
    struct A {}
}

#[tokio::main]
async fn main() {
    let router = router().arced();

    router
        .export_ts(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../client/src/bindings.ts"))
        .unwrap();

    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest("/rspc", rspc_axum::endpoint(router, || ()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    // It is highly recommended to unit test your rspc router by creating it
    // This will ensure it doesn't have any issues and also export updated Typescript types.

    #[test]
    fn test_rspc_router() {
        super::router();
    }
}
