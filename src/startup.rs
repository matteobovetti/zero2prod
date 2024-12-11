use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Database pool
    let db_pool = web::Data::new(db_pool);

    // Start https server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route(
                "/health_check",
                web::get().to(crate::routes::health_check::health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(crate::routes::subscriptions::subscribe),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
