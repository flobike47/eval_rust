use actix_web::{middleware, App, HttpServer};
use eval_rust::configure_routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
            .wrap(middleware::Logger::default())
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}


