use std::{path::PathBuf, time::Duration};

use axum::routing::get;
use rspc::Router;
use serde::{Deserialize, Serialize};
use specta::Type;
use tokio::time::sleep;
use tower_http::cors::CorsLayer;

#[derive(Type, Deserialize, Serialize)]
struct User {
    mobile: String,
    more: i32,
    what: Option<Vec<bool>>,
}

fn router() -> Router<()> {
    <Router>::new()
        .query("version", |t| t(|ctx, input: ()| env!("CARGO_PKG_VERSION")))
        .mutation("toggle", |t| {
            t(|ctx, mut input: User| {
                input.mobile += " 1";
                input
            })
        })
        .subscription("listen_new_user", |t| {
            t(|ctx, input: User| {
                async_stream::stream! {
                    for i in input.more.. {
                        yield User { mobile: "m".into(), more: i, what: None };
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            })
        })
        .build()
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
