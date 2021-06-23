use actix_web::{web, Responder};
use crate::models::Status;

pub async fn status() -> impl Responder{
    web::HttpResponse::Ok()
        .json(Status {status:"Running".to_string()})
}