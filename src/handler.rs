// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::{
    model::{Db, QueryOptions, Todo, UpdateTodoSchema},
    respose::{SingleTodoResponse, TodoData, TodoListResponse},
};

#[instrument]
pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_res = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    trace!(
        "Called 'GET /api/healthcheck' successfully, returning {}",
        json_res
    );

    Json(json_res)
}

// GET /api/todos ?page ?limit
// Ok: <none>
// Err: <none>
#[instrument]
pub async fn todo_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<Db>,
) -> impl IntoResponse {
    let Query(opts) = opts.unwrap_or_default();
    let todos = db.lock().await;

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_res = TodoListResponse {
        status: "success".into(),
        results: todos.len(),
        todos,
    };

    trace!(
        "Called 'GET /api/todos' successfully, returnig {:#?}",
        json_res
    );

    Json(json_res)
}

// GET /api/todos/:id
// Ok: OK
// Err: NOT_FOUND
#[instrument]
pub async fn get_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.id == Some(id.to_owned())) {
        let res = SingleTodoResponse {
            status: "success".into(),
            data: TodoData { todo: todo.clone() },
        };

        trace!(
            "Called 'GET /api/todos/{}' successfully, returning {:#?}",
            id,
            res
        );

        return Ok((StatusCode::OK, Json(res)));
    }

    let err_res = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id),
    });

    trace!(
        "Called 'GET /api/todos/{}' unsuccessfully, returning {:#?}",
        id,
        err_res
    );

    Err((StatusCode::NOT_FOUND, Json(err_res)))
}

// POSt __________ Todo
// Ok: CREATED,
// Err: CONFLICT
#[instrument]
pub async fn create_todo_handler(
    State(db): State<Db>,
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.title == body.title) {
        let err_res = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title: '{}' already exists", todo.title)
        });

        trace!(
            "Called 'POST /api/todos' unsuccessfully, returning {:#?}",
            err_res
        );

        return Err((StatusCode::CONFLICT, Json(err_res)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.created_at = Some(datetime);
    body.updated_at = Some(datetime);

    let todo = body.to_owned();
    vec.push(body.clone()); // TODO: somehow remove clone (It's used from trace!())

    let json_res = SingleTodoResponse {
        data: TodoData { todo },
        status: "success".into(),
    };

    trace!(
        "Called 'POST /api/todos' successfully, returning {:#?}",
        json_res
    );

    Ok((StatusCode::CREATED, Json(json_res)))
}

// PATCH /api/todos/:id UpdateTodoSchema
// Ok: OK
// Err: NOT_FOUND
#[instrument]
pub async fn edit_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
    Json(body): Json<UpdateTodoSchema>,
) -> Result<(StatusCode, Json<SingleTodoResponse>), (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter_mut().find(|todo| todo.id == Some(id.clone())) {
        let datetime = chrono::Utc::now();

        let title = body
            .title
            .to_owned()
            .unwrap_or_else(|| todo.title.to_owned());

        let content = body
            .content
            .to_owned()
            .unwrap_or_else(|| todo.content.to_owned());
        let completed = body.completed.unwrap_or(todo.completed.unwrap());

        let payload = Todo {
            id: todo.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                todo.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                todo.content.to_owned()
            },
            completed: Some(completed),
            created_at: todo.created_at,
            updated_at: Some(datetime),
        };

        *todo = payload;

        let res = SingleTodoResponse {
            status: "success".into(),
            data: TodoData { todo: todo.clone() },
        };

        trace!(
            "Called 'PATCH /api/todos/{}' successfully, returning {:#?}",
            id,
            res
        );

        Ok((StatusCode::OK, Json(res)))
    } else {
        let err_res = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with ID: {} not found", id),
        });

        trace!(
            "Called 'PATCH /api/todos/{}' unsuccessfully, returning {:#?}",
            id,
            err_res
        );

        Err((StatusCode::NOT_FOUND, Json(err_res)))
    }
}

// DELETE /api/todos/:id
// Ok: NO_CONTENT
// Err: NOT_FOUND
#[instrument]
pub async fn delete_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(|todo| todo.id == Some(id.clone())) {
        vec.remove(pos);

        trace!(
            "Called 'DELETE /api/todos/{}' successfully, returning {:#?}",
            id,
            ""
        );

        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let err_res = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id),
    });

    trace!(
        "Called 'DELETE /api/todos/{}' unsuccessfully, returning {:#?}",
        id,
        err_res
    );

    Err((StatusCode::NOT_FOUND, Json(err_res)))
}
