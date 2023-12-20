use std::net::SocketAddr;

use http_body_util::{BodyExt, Full};
use hyper::{header, Method, Request, Response, StatusCode};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |request| handler(request));
            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Failed to serve connection: {:?}", err)
            }
        });
    }
}

async fn handler(req: Request<Incoming>) -> Result<Response<BoxBody>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => get_root().await,
        (&Method::GET, "/200") => get_ok().await,
        (&Method::GET, "/404") => get_not_found().await,
        (&Method::GET, "/500") => get_internal_server_error().await,
        _ => get_not_found().await,
    }
}

const INDEX: &[u8] = b"<!DOCTYPE html><html><em>Hello, world</em>";

async fn get_root() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .body(full(INDEX)).unwrap())
}

async fn get_ok() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(full(StatusCode::OK.as_str())).unwrap())
}

const NOTFOUND: &[u8] = b"404 page not found";

async fn get_not_found() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(NOTFOUND))
        .unwrap())
}

const INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

async fn get_internal_server_error() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(full(INTERNAL_SERVER_ERROR))
        .unwrap())
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
