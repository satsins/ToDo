use crate::models::{TodoList, TodoItem};
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;
use deadpool_postgres::Client;
use crate::errors::{AppError, AppErrorType};

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, AppError> {
    let statement = client.prepare("select * from todo_list order by id desc").await
        .map_err(AppError::db_error)?;

    let todos = client.query(&statement, &[])
        .await.expect("Error getting todo_list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, AppError> {
    let statement = client.prepare("select * from todo_item where list_id = $1 order by id").await
        .map_err(AppError::db_error)?;

    let items = client.query(&statement, &[&list_id])
        .await.expect("Error getting todo_item")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(items)
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, AppError> {
    let statement = client.prepare("insert into todo_list (title) values ($1) returning id, title").await
        .map_err(AppError::db_error)?;

    client.query(&statement, &[&title])
        .await.expect("Error creating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(
            AppError{
                message: Some("Error creating todo list".to_string()),
                cause: Some("unknown error".to_string()),
                error_type: AppErrorType::DbError
            }
        )
}

pub async fn check_todo(client: &Client, list_id: i32, item_id: i32) -> Result<bool, AppError> {
    let statement = client
        .prepare("update todo_item set checked = true where list_id = $1 and id = $2 and checked = false").await
        .map_err(AppError::db_error)?;

    let result = client.execute(&statement, &[&list_id, &item_id]).await.unwrap();
    match result {
        ref updated if *updated == 1 => Ok(true),
        _ => Ok(false),
    }
}