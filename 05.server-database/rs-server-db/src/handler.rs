use actix_web::{get, HttpResponse, post, Responder, web};
use serde::Serialize;

use crate::{dto, entity};
use crate::database::Database;
use crate::error::ErrorResponse;

#[get("/images.json")]
pub async fn get_image(info: web::Query<dto::ImageRequest>, db: web::Data<Database>) -> impl Responder {
    let images: Vec<_> = db.fetch_images()
        .iter()
        .cloned()
        .map(|img| dto::Image::from(img))
        .collect();

    marshal_json(images, info.indent)
}

#[post("/images.json")]
pub async fn post_image(info: web::Query<dto::ImageRequest>, db: web::Data<Database>, payload: web::Json<dto::Image>) -> impl Responder {
    let image = payload.0;
    let new_image: entity::NewImage = image.clone().into();

    if db.is_duplicated_image(new_image.clone()) {
        return ErrorResponse::duplicate_error().http_response();
    }

    db.save_image(new_image);
    marshal_json(image, info.indent)
}

fn marshal_json<T>(item: T, indent: Option<usize>) -> HttpResponse
    where
        T: Serialize
{
    match indent {
        Some(indent) => {
            let mut buf = Vec::new();
            let indent_str = " ".repeat(indent);
            let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_str.as_bytes());
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            item.serialize(&mut ser).unwrap();
            HttpResponse::Ok().insert_header(("Content-Type", "application/json")).body(buf)
        }
        None => HttpResponse::Ok().json(item)
    }
}
