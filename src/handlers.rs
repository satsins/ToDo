use actix_web::{web, Responder, HttpResponse};
use crate::models::{Status, CreateTodoList, ResultResponse};
use deadpool_postgres::{Pool,Client};
use crate::db;
use std::io::ErrorKind::Other;
use std::io::Error;
use crate::errors::{AppError, AppErrorType};

pub async fn status() -> impl Responder{
    HttpResponse::Ok()
        .json(Status {status:"Running".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError>{
    let client:Client = db_pool.get().await.map_err(AppError::db_error)?;

    let result = db::get_todos(&client).await;
    result.map(|todos| HttpResponse::Ok().json(todos))
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError>{
    let client:Client = db_pool.get().await.map_err(AppError::db_error)?;

    let path = path.into_inner();
    let result = db::get_items(&client, path.0).await;
    result.map(|items| HttpResponse::Ok().json(items))
}

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> Result<impl Responder, AppError>{
    let client:Client = db_pool.get().await.map_err(AppError::db_error)?;

    let result = db::create_todo(&client, json.title.clone()).await;
    result.map(|todos| HttpResponse::Ok().json(todos))

}

pub async fn check_item(db_pool: web::Data<Pool>, params: web::Path<(i32, i32)>) -> Result<impl Responder, AppError> {
    let client:Client = db_pool.get().await.map_err(AppError::db_error)?;

    let params = params.into_inner();
    let result = db::check_todo(&client, params.0, params.1).await;
    result.map(|updated| HttpResponse::Ok().json(ResultResponse{success: updated}))
}