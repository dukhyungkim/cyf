use actix_web::{App, HttpServer};
use rs_server_db::handler::get_image;

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
