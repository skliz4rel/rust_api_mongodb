use crate::{
    models::owner_model::{Owner, OwnerRequest},
    services::db::Database,
};
use actix_web::post;
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};

#[post("/owner")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
    match db
        .create_owner(
            Owner::try_from(OwnerRequest {
                name: request.name.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                address: request.address.clone(),
            })
            .expect("Error converting DogRequest to Dog."),
        )
        .await
    {
        Ok(dog) => HttpResponse::Ok().json(dog),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
