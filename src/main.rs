use actix_files as fs;
use actix_web::{App, HttpResponse, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./public").index_file("index.html"))
            .default_service(web::to(async || {
                HttpResponse::NotFound().body("404 Not Found")
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
