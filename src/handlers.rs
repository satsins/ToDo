use actix_web::{web, Responder, HttpResponse};
use crate::models::{Status, CreateTodoList, ResultResponse};
use deadpool_postgres::{Pool,Client};
use crate::db;

pub async fn status() -> impl Responder{
    HttpResponse::Ok()
        .json(Status {status:"Running".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder{
    let client:Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder{
    let client:Client = db_pool.get().await.expect("Error connecting to database");
    let path = path.into_inner();
    let result = db::get_items(&client, path.0).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder{
    let client:Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::create_todo(&client, json.title.clone()).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_todo(db_pool: web::Data<Pool>, params: web::Path<(i32, i32)>) -> impl Responder {
    let client:Client = db_pool.get().await.expect("Error connecting to database");
    let params = params.into_inner();
    let result = db::check_todo(&client, params.0, params.1).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(ResultResponse{success: true}),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}