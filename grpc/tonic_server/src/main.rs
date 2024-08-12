use std::result::Result;

use tonic::{transport::Server, Request, Response, Status};

use tttt::{
    route_test_server::{RouteTest, RouteTestServer},
    ColorPoint, Point,
};

pub mod tttt {
    tonic::include_proto!("tttt");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl RouteTest for MyGreeter {
    async fn check_color_point(
        &self,
        request: Request<ColorPoint>,
    ) -> Result<Response<ColorPoint>, Status> {
        let mut color_point = request.into_inner();
        if let Some(point) = color_point.point.as_mut() {
            point.x += 100;
        }

        Ok(Response::new(color_point))
    }

    async fn check_point(&self, request: tonic::Request<Point>) -> Result<Response<Point>, Status> {
        Ok(Response::new(request.into_inner()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse().unwrap();

    let route_test = MyGreeter::default();
    let route_test = RouteTestServer::new(route_test);

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        // GrpcWeb is over http1 so we must enable it.
        .accept_http1(true)
        .add_service(tonic_web::enable(route_test))
        .serve(addr)
        .await?;

    Ok(())
}
