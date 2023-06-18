// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::trace;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub type Db = Arc<Mutex<Vec<Todo>>>;

/// Creates a new database pool
pub fn todo_db() -> Db {
    trace!("Created new memory Db pool");
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
