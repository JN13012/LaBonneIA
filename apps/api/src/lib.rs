mod routes;

pub fn app() -> axum::Router {
    routes::router()
}
