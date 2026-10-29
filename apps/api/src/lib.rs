mod routes;
mod tts;

pub fn app() -> axum::Router {
    routes::router()
}
