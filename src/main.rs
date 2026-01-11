use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};

mod models;
mod routes;
mod services;

use crate::routes::booking_route::*;
use crate::routes::dog_route::*;
use crate::routes::owner_route::*;
use crate::services::db::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello YouTube!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) //register or inject the database obj
            .service(hello)
            .service(create_booking)
            .service(create_dog)
            .service(create_owner)
            .service(get_bookings)
            .service(cancel_bookings)
    })
    .bind(("localhost", 5001))?
    .run()
    .await
}
