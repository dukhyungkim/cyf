use hyper::{Method, Request, Response};
use hyper::body::Incoming;

use crate::handler::{get_authenticated, get_ok, get_root, internal_server_error, not_found, post_root};
use crate::types;
use crate::types::BoxBody;

pub async fn router(req: Request<Incoming>) -> types::Result<Response<BoxBody>> {
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
