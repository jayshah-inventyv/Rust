use axum::Router;

// use crate::

pub fn get_routes() -> Router {
    Router::new().merge(get_status_routes())
}
