

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::fs;
use std::collections::HashMap;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut paths = HashMap::new();
    paths.insert("/foo", StatusCode::OK);
    paths.insert("/bar", StatusCode::FOUND);
    paths.insert("/baz", StatusCode::NOT_FOUND);
    paths.insert("/qux", StatusCode::INTERNAL_SERVER_ERROR);

    match (req.method(), req.uri().path()) {
        (&Method::GET, path) => {
            let status = paths.get(path).cloned().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let body = format!(
                "<html><head><title>Rust Server</title></head>
                <body><p>This is a test.</p>
                <p>You accessed path: {}</p></body></html>", 
                path
            );
            Ok(Response::builder()
                .status(status)
                .header("Content-Type", "text/html")
                .body(Body::from(body))
                .unwrap())
        }
        (&Method::POST, _) => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let post_data = String::from_utf8(whole_body.to_vec()).unwrap_or_else(|_| "Invalid UTF-8".to_string());
            println!("Received POST Data: {}", post_data);

            Ok(Response::new(Body::from(format!("POST request received: {}", post_data))))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("Method Not Allowed"))
            .unwrap()),
    }
}


async fn run() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 9999));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("🚀 Rust HTTP Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
