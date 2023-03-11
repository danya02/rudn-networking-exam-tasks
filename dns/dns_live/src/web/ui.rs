use axum::{response::Html, extract::Path, http::StatusCode};
use super::templates::env;
use minijinja::context;

use crate::session::get_session;

pub async fn session_get(Path(key): Path<String>) -> (StatusCode, Html<String>) {
    let env = env();
    let session = get_session(&key).await;
    if let None = session {
        return (StatusCode::NOT_FOUND, Html(env.get_template("sessionnotfound").unwrap().render(context!(key => key)).unwrap()));
    }
    let session = session.unwrap();
    let html = env.get_template("session").unwrap().render(context!(session => session)).unwrap();
    return (StatusCode::OK, Html(html));
}
pub async fn session_post(Path(key): Path<String>) -> Html<String> {
    Html("HelloWorld!".to_string())
}