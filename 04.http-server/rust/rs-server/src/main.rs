use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use base64::Engine;
use http_body_util::{BodyExt, Empty, Full};
use hyper::{header, Method, Request, Response, StatusCode};
use hyper::body::{Bytes, Incoming};
use hyper::body::Buf;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |request| router(request));
            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Failed to serve connection: {:?}", err)
            }
        });
    }
}

async fn router(req: Request<Incoming>) -> Result<Response<BoxBody>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => get_root(req).await,
        (&Method::POST, "/") => post_root(req).await,
        (&Method::GET, "/200") => get_ok().await,
        (&Method::GET, "/404") => Ok(not_found()),
        (&Method::GET, "/500") => Ok(internal_server_error()),
        (&Method::GET, "/authenticated") => get_authenticated(req).await,
        _ => Ok(not_found()),
    }
}

const HTML: &[u8] = b"<!DOCTYPE html><html>";

const HTML_HELLO: &[u8] = b"<!DOCTYPE html><html><em>Hello, world</em>";

async fn get_root(req: Request<Incoming>) -> Result<Response<BoxBody>> {
    let query = if let Some(q) = req.uri().query() {
        q
    } else {
        return Ok(Response::builder()
            .header(header::CONTENT_TYPE, "text/html")
            .body(full(HTML_HELLO)).unwrap());
    };

    let params = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let foo = params.get("foo").unwrap();
    let html_foo = format!("<!DOCTYPE html>
<html>
<em>Hello, world</em>
<p>Query parameters:
<ul>
<li>foo: {}</li>
</ul>", foo);
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .body(full(html_foo)).unwrap())
}

async fn post_root(req: Request<Incoming>) -> Result<Response<BoxBody>> {
    let body = req.collect().await?.aggregate();
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .body(full([HTML, body.chunk()].concat())).unwrap())
}

async fn get_ok() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(full(StatusCode::OK.as_str())).unwrap())
}


async fn get_authenticated(req: Request<Incoming>) -> Result<Response<BoxBody>> {
    const BASIC_PREFIX: &str = "Basic ";

    let auth = if let Some(auth) = req.headers().get("Authorization") {
        auth
    } else {
        return Ok(un_authorization());
    };

    let auth_str = auth.to_str().unwrap();
    if auth_str.starts_with(BASIC_PREFIX) {
        let credentials = auth_str.trim_start_matches(BASIC_PREFIX);
        let decoder = base64::engine::general_purpose::STANDARD;
        let decoded = decoder.decode(credentials).unwrap();
        let creeds = String::from_utf8(decoded).unwrap();
        let user_pass: Vec<_> = creeds.split(":").collect();

        let username = user_pass[0];
        if creeds == "username:password" {
            return Ok(Response::builder()
                .body(full([HTML, username.as_bytes()].concat()))
                .unwrap());
        }
    }

    return Ok(un_authorization());
}

fn not_found() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(b"404 page not found".as_slice()))
        .unwrap()
}

fn internal_server_error() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(full(b"Internal Server Error".as_slice()))
        .unwrap()
}

fn un_authorization() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("WWW-Authenticate", "Basic realm=\"Restricted Area\"")
        .body(empty())
        .unwrap()
}

fn empty() -> BoxBody {
    Empty::<Bytes>::new().boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into()).boxed()
}
