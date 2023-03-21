use std::{net::SocketAddr, convert::Infallible};

use hyper::{service::{make_service_fn, service_fn}, Body, Response, Request, Server};

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr)
        .http1_keepalive(false)
        .serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
