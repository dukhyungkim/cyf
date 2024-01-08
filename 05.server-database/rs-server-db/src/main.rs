use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use rs_server_db::image::images;

#[get("/images.json")]
async fn get_image() -> impl Responder {
    HttpResponse::Ok().json(images())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listen and serve: 0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .service(get_image)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
