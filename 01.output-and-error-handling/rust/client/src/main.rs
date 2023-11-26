use std::{process, thread, time};

use chrono::{DateTime, Utc};
use reqwest::StatusCode;

const SERVER_ADDRESS: &str = "http://localhost:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = match reqwest::get(SERVER_ADDRESS).await {
        Ok(resp) => resp,
        Err(_) => {
            println!("failed to get weather.");
            process::exit(1);
        }
    };

    let status_code = resp.status();
    let headers = resp.headers().clone();
    let body = resp.text().await?;

    match status_code {
        StatusCode::OK => println!("{}", body),
        StatusCode::TOO_MANY_REQUESTS => {
            let retry_after_header = headers.get("Retry-After").unwrap();
            let retry_after_header = retry_after_header.to_str()?.to_string();

            match retry_after_header.parse::<u64>() {
                Ok(retry_after) => wait_and_retry(retry_after),
                Err(_) => {
                    match DateTime::parse_from_rfc2822(&retry_after_header) {
                        Ok(dt) => {
                            let dt = dt.with_timezone(&Utc);
                            let retry_after = dt - Utc::now();
                            wait_and_retry(retry_after.num_seconds() as u64);
                        }
                        Err(_) => {
                            println!("can't determine how long to sleep: {}", retry_after_header);
                        }
                    }
                }
            }
        }
        _ => println!("cannot be here"),
    }

    Ok(())
}

fn wait_and_retry(retry_after: u64) {
    if retry_after < 5 {
        println!("try again after {} seconds...", retry_after);
        thread::sleep(time::Duration::from_secs(retry_after));
        println!("retry");
    } else {
        println!("wait too long time({} seconds). can't get the weather.", retry_after);
    }
}
