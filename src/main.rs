use actix_web::{App, HttpServer, web};

mod secure;
mod vulnerable;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async {
                    "Welcome to the SSRF demo! Visit /vulnerable or /secure endpoints."
                }),
            )
            .service(web::scope("/vulnerable").route(
                "/fetch_resource",
                web::post().to(vulnerable::fetch_resource),
            ))
            .service(
                web::scope("/secure")
                    .route("/fetch_resource", web::post().to(secure::fetch_resource)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
