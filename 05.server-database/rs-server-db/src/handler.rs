use actix_web::{get, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use crate::image::images;

#[derive(Debug, Deserialize)]
struct ImageRequest {
    indent: Option<usize>,
}

#[get("/images.json")]
pub async fn get_image(info: web::Query<ImageRequest>) -> impl Responder {
    let images = images();

    match info.indent {
        Some(indent) => {
            let mut buf = Vec::new();
            let indent_str =  " ".repeat(indent);
            let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_str.as_bytes());
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            images.serialize(&mut ser).unwrap();
            HttpResponse::Ok().insert_header(("Content-Type", "application/json")).body(buf)
        },
        None => HttpResponse::Ok().json(images)
    }
}
