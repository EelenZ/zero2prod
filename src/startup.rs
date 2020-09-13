use std::{net::TcpListener, sync::Arc};

use sqlx::{PgConnection, PgPool};
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;

use crate::routes::subscribe;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool in an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Our pool is already wrapped in an Arc pointer: 
            // using .data would add another Arc pointer on top 
            // of the existing one - an unnecessary indirection.
            // .app_data instead does not perform an additional layer of wrapping.
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}