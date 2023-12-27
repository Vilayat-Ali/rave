mod handlers;

use axum::routing::{delete, get, post, put};
use axum::Router;

use super::chat::handlers::*;

pub struct Chat;

impl Chat {
    pub fn generate_routes() -> Router {
        Router::new()
            // CREATE
            .route("/", post(handle_create))
            // READ ALL
            .route("/all", get(handle_read_all))
            // READ ONE
            .route("/:id", get(handle_read_one))
            // UPDATE
            .route("/:id", put(handle_update))
            // DELETE
            .route("/:id", delete(handle_delete))
    }
}
