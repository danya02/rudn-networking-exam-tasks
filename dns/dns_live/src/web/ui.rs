use std::net::{IpAddr, SocketAddr};

use super::templates::{env, SessionRequest};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, Redirect},
    Form,
};
use chrono::Utc;
use minijinja::context;

use crate::{
    querying::{is_recursive_server, perform_query, validate_answer},
    session::{get_session, set_session, AnswerStatus, Event, Request, RequestLogEntry, Response},
};

pub async fn session_get(Path(key): Path<String>) -> (StatusCode, Html<String>) {
    let env = env();
    let session = get_session(&key).await;
    if session.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Html(
                env.get_template("sessionnotfound")
                    .unwrap()
                    .render(context!(key => key))
                    .unwrap(),
            ),
        );
    }
    let session = session.unwrap();
    let html = env
        .get_template("session")
        .unwrap()
        .render(context!(session => session))
        .unwrap();
    (StatusCode::OK, Html(html))
}
pub async fn session_post(
    Path(key): Path<String>,
    Form(request): Form<SessionRequest>,
) -> Result<Redirect, (StatusCode, Html<String>)> {
    let env = env();
    let session = get_session(&key).await;
    if session.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Html(
                env.get_template("sessionnotfound")
                    .unwrap()
                    .render(context!(key => key))
                    .unwrap(),
            ),
        ));
    }
    let mut session = session.unwrap();
    if !session.can_answer {
        return Err((
            StatusCode::BAD_REQUEST,
            Html(
                env.get_template("noattempts")
                    .unwrap()
                    .render(context!(key => key))
                    .unwrap(),
            ),
        ));
    }

    let event = match request {
        SessionRequest::Query { ip, class, name } => {
            let ip = ip.trim();
            let name = name.trim();
            let request = Request {
                server_ip: ip.to_owned(),
                name: name.to_owned(),
                record_type: class,
            };
            // Try parsing the query IP address.
            let maybe_parsed_ip = ip.parse::<IpAddr>();
            if let Ok(parsed_ip) = maybe_parsed_ip {
                // First perform the query the user asked for.
                let ip = SocketAddr::new(parsed_ip, 53);
                let true_response = perform_query(ip, &name, class).await;
                match true_response {
                    Ok(resp) => {
                        // We have a response, so the server exists.
                        // But is it a recursive server?
                        if is_recursive_server(ip).await {
                            Event::Request {
                                request,
                                response: crate::session::ResponseResult::ForbiddenRecursion {
                                    addr: ip.ip(),
                                },
                            }
                        } else {
                            let text = match session.current_output_mode {
                                crate::session::OutputMode::Classic => resp.to_string(),
                                crate::session::OutputMode::Rust => format!("{resp:#?}"),
                            };
                            Event::Request {
                                request,
                                response: crate::session::ResponseResult::Ok {
                                    resp: Response {
                                        text,
                                        mode: session.current_output_mode,
                                    },
                                },
                            }
                        }
                    }
                    Err(error) => Event::Request {
                        request,
                        response: crate::session::ResponseResult::QueryError {
                            err: format!("{error:?}"),
                        },
                    },
                }
            } else {
                // Add error record
                Event::Request {
                    request,
                    response: crate::session::ResponseResult::InvalidRequestIpAddr {
                        addr: ip.to_owned(),
                    },
                }
            }
        }
        SessionRequest::SetOutputMode { mode } => {
            session.current_output_mode = mode;
            Event::SwitchOutputMode { new_mode: mode }
        }
        SessionRequest::SubmitAnswer { answer } => {
            let answer = answer.trim().to_owned();
            let status = match validate_answer(&session.question.answer, &answer).await {
                Some(true) => AnswerStatus::Correct,
                Some(false) => AnswerStatus::Incorrect,
                None => AnswerStatus::Error,
            };
            if !matches!(status, AnswerStatus::Error) {
                session.answers_remaining -= 1;
            }

            if matches!(status, AnswerStatus::Correct) || session.answers_remaining==0 {
                session.can_answer = false;
            }


            Event::SubmitAnswer { answer, status }
        }
    };
    session.user_requests.push(RequestLogEntry {
        when: Utc::now(),
        what: event,
    });

    if set_session(&key, session).await.is_none() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(
                env.get_template("errorsaving")
                    .unwrap()
                    .render(context!(key => key))
                    .unwrap(),
            ),
        ));
    }
    Ok(Redirect::to(&format!("/{key}#new-query")))
}
