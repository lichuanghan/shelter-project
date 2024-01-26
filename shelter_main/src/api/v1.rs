use axum::Router;
use axum::routing::get;

use super::handlers;

pub fn configure() -> Router {
    Router::new().route("/hello", get(handlers::hello::hello))
}