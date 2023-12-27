use std::collections::HashMap;
use std::env;

use base64::Engine;
use http_body_util::{BodyExt, Empty, Full};
use hyper::{header, Request, Response, StatusCode};
use hyper::body::{Bytes, Incoming};
use hyper::body::Buf;

use crate::types;
use crate::types::BoxBody;

const HTML: &[u8] = b"<!DOCTYPE html><html>";

const HTML_HELLO: &[u8] = b"<!DOCTYPE html><html><em>Hello, world</em>";

pub async fn get_root(req: Request<Incoming>) -> types::Result<Response<BoxBody>> {
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

pub async fn post_root(req: Request<Incoming>) -> types::Result<Response<BoxBody>> {
    let body = req.collect().await?.aggregate();
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .body(full([HTML, body.chunk()].concat())).unwrap())
}

pub async fn get_ok() -> types::Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(full(StatusCode::OK.as_str())).unwrap())
}

pub async fn get_authenticated(req: Request<Incoming>) -> types::Result<Response<BoxBody>> {
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

        let cred = get_credential();
        if creeds == cred {
            return Ok(Response::builder()
                .body(full([HTML, username.as_bytes()].concat()))
                .unwrap());
        }
    }

    return Ok(un_authorization());
}

pub fn not_found() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(b"404 page not found".as_slice()))
        .unwrap()
}

pub fn internal_server_error() -> Response<BoxBody> {
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

pub async fn too_many_requests() -> types::Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .body(full(StatusCode::TOO_MANY_REQUESTS.as_str())).unwrap())
}

fn empty() -> BoxBody {
    Empty::<Bytes>::new().boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into()).boxed()
}

fn get_credential() -> String {
    let username = env::var("AUTH_USERNAME").unwrap_or("username".to_string());
    let password = env::var("AUTH_PASSWORD").unwrap_or("password".to_string());
    format!("{}:{}", username, password)
}
