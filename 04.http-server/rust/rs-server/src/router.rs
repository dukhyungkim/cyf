use std::sync::{Arc, Mutex};

use hyper::{Method, Request, Response};
use hyper::body::Incoming;

use crate::handler::{get_authenticated, get_ok, get_root, internal_server_error, not_found, post_root, too_many_requests};
use crate::middleware::RateLimiter;
use crate::types;
use crate::types::BoxBody;

pub async fn router(req: Request<Incoming>) -> types::Result<Response<BoxBody>> {
    let rate_limiter = Arc::new(Mutex::new(RateLimiter::new()));

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => get_root(req).await,
        (&Method::POST, "/") => post_root(req).await,
        (&Method::GET, "/200") => get_ok().await,
        (&Method::GET, "/404") => Ok(not_found()),
        (&Method::GET, "/500") => Ok(internal_server_error()),
        (&Method::GET, "/authenticated") => get_authenticated(req).await,
        (&Method::GET, "/limited") => {
            let mut rate_limiter = rate_limiter.lock().unwrap();
            if rate_limiter.is_allowed() {
                get_ok().await
            } else {
                too_many_requests().await
            }
        }
        _ => Ok(not_found()),
    }
}
