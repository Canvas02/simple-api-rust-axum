// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use axum::{
    routing::{get, post},
    Router,
};
use tracing::trace;

use crate::{
    handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        health_check_handler, todo_list_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::todo_db();

    let router = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route(
            "/api/todos",
            post(create_todo_handler).get(todo_list_handler),
        )
        .route(
            "/api/todos/:id",
            get(get_todo_handler)
                .patch(edit_todo_handler)
                .delete(delete_todo_handler),
        )
        .with_state(db);

    trace!("Created router");

    router
}
