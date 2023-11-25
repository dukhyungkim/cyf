use actix_web::{App, get, HttpResponse, HttpServer};
use chrono::{Duration, Utc};
use rand::Rng;

#[get("/")]
async fn handle_request() -> HttpResponse {
    let random_number = rand::thread_rng().gen_range(0..10);

    match random_number {
        0..=2 => HttpResponse::Ok().body("Today it will b sunny!"),
        3..=4 => HttpResponse::Ok().body("I'd bring an umbrella, just in case..."),
        5 => {
            let retry_after_seconds = rand::thread_rng().gen_range(1..=10);
            reject_as_too_busy(&retry_after_seconds.to_string())
        },
        6 => {
            let retry_after_seconds = rand::thread_rng().gen_range(1..=10);

            let time_after_delay = Utc::now()+ Duration::seconds(retry_after_seconds);
            const RFC1123: &str = "%a, %d %b %Y %H:%M:%S %Z";

            let retry_after_header = time_after_delay.format(RFC1123);
            reject_as_too_busy(&retry_after_header.to_string())
        },
        7 => {
            const RETRY_AFTER: &str = "a while";
            reject_as_too_busy(RETRY_AFTER)
        },
        8..=9 => {
            // difficult to make socket hang up situation
            HttpResponse::BadRequest().force_close().finish()
        },
        default => {
            println!("Reached unreachable code - HTTP handler switch encountered unhandled random number {} which shouldn't be possible", default);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

fn reject_as_too_busy(retry_after_header: &str) -> HttpResponse {
    HttpResponse::TooManyRequests()
        .insert_header(("Retry-After", retry_after_header))
        .body("Sorry, I'm too busy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port 8080...");
    HttpServer::new(|| {
        App::new()
            .service(handle_request)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
