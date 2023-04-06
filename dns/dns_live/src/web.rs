use axum::{response::{Html, Redirect}, routing::get, Router, Form};
use minijinja::context;

use self::templates::{env, SessionKeyRequest};

mod templates;
mod ui;

pub async fn web_main() -> ! {
    let app = get_router();

    let server =
        axum::Server::bind(&"0.0.0.0:5000".parse().unwrap()).serve(app.into_make_service());
    if let Err(e) = server.await {
        panic!("Server error: {e:?}");
    } else {
        unreachable!("Server stopped on its own?!");
    }
}

fn get_router() -> Router {
    Router::new()
        .route("/", get(home).post(route_to_session))
        .fallback(not_found)
        .route("/:session", get(ui::session_get).post(ui::session_post))
}

async fn home() -> Html<String> {
    let env = env();
    Html(
        env.get_template("home")
            .unwrap()
            .render(context!())
            .unwrap(),
    )
}

async fn route_to_session(
    Form(request): Form<SessionKeyRequest>,
) -> Redirect {
    Redirect::to(&("/".to_string() + &request.key))
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
