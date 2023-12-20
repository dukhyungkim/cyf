use std::net::SocketAddr;

use http_body_util::{BodyExt, Full};
use hyper::{Method, Request, Response, StatusCode};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"404 page not found";

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

async fn get_root() -> Result<Response<BoxBody>> {
    Ok(Response::builder().body(full("Hello, World!")).unwrap())
}

async fn get_ok() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(full(StatusCode::OK.as_str())).unwrap())
}

async fn get_not_found() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(NOTFOUND))
        .unwrap())
}

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
