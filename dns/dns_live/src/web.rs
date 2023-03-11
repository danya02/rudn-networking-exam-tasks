use axum::{response::Html, routing::get, Router};
use minijinja::context;

use self::templates::env;

mod templates;
mod ui;

pub async fn web_main() -> ! {
    let app = get_router();

    let server =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());
    if let Err(e) = server.await {
        panic!("Server error: {e:?}");
    } else {
        unreachable!("Server stopped on its own?!");
    }
}

fn get_router() -> Router {
    Router::new()
        .route("/", get(not_found))
        .fallback(not_found)
        .route("/:session", get(ui::session_get).post(ui::session_post))
}

async fn not_found() -> Html<String> {
    let env = env();
    Html(
        env.get_template("notfound")
            .unwrap()
            .render(context!())
            .unwrap(),
    )
}
