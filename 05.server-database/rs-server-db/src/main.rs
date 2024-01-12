use actix_web::{App, HttpServer, web};
use rs_server_db::database::Database;

use rs_server_db::handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new();

    println!("Listen and serve: 0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(handler::get_image)
            .service(handler::post_image)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
