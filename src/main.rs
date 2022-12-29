use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Compress, Logger};

mod route;


/// boot_application_server - will kick start the application service tha
async fn boot_application_server() -> std::io::Result<()> {
    HttpServer::new(move|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(route::general_config)
            .service(
                web::scope("/v1")
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Boot Starting the Application!");
    boot_application_server().await
}
