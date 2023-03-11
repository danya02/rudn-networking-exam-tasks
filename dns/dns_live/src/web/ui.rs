use axum::{response::Html, extract::Path, http::StatusCode};

use crate::session::get_session;

pub async fn session_get(Path(key): Path<String>) -> (StatusCode, Html<String>) {
    let session = get_session(&key).await;
    if let None = session {
        return (StatusCode::NOT_FOUND, Html(format!("No such session found: {key}")));
    }
    return (StatusCode::OK, Html("ok".to_string()));
}
pub async fn session_post(Path(key): Path<String>) -> Html<String> {
    Html("HelloWorld!".to_string())
}