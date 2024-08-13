use poem::{http::Method, listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_openapi::{
    param::{Path, Query},
    payload::{Json, PlainText},
    Object, OpenApi, OpenApiService,
};
use serde::Deserialize;

#[derive(Object, Debug, Default, Deserialize)]
pub struct SendCode {
    pub mobile: String,
}

#[derive(Object, Debug)]
pub struct SendCodeResponse {
    pub found_account: bool,
}

#[derive(Default)]
struct Api {}

#[OpenApi]
impl Api {
    #[oai(path = "/send_code", method = "post")]
    async fn send_code(&self, _body: Json<SendCode>) -> Json<SendCodeResponse> {
        Json(SendCodeResponse {
            found_account: false,
        })
    }

    #[oai(path = "/send_code2/:path", method = "post")]
    async fn send_code2(
        &self,
        Path(path): Path<String>,
        Query(query): Query<String>,
        Query(query2): Query<Vec<u8>>,
        Json(data): Json<SendCode>,
    ) -> Json<String> {
        Json(format!(
            "path: \n{path:?}\nquery: {query:?}\nquery2: {query:?}\ndata: {data:?} "
        ))
    }

    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {name}!")),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_credentials(false);

    let api_service =
        OpenApiService::new(Api::default(), "Users", "1.0").server("http://localhost:3000/");

    std::fs::write("openapi.yaml", api_service.spec_yaml()).expect("export openapi spec fail");

    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(
            Route::new()
                .nest("/", api_service)
                .nest("/ui/", ui)
                .with(cors),
        )
        .await
}
