use minijinja::Environment;

pub fn env() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_template("theme", THEME).unwrap();
    env.add_template("notfound", NOT_FOUND).unwrap();
    env.add_template("sessionnotfound", SESSION_NOT_FOUND).unwrap();
    env.add_template("session", SESSION).unwrap();
    env
}

const THEME: &str = r#"
<!doctype html>
<html>
    <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-GLhlTQ8iRABdZLl6O3oVMWSktQOp6b7In1Zl3/Jr59b6EGGoI1aFkw7cmDA6j6gD" crossorigin="anonymous">
    {% block head %}{% endblock %}
    </head>
    <body data-bs-theme="dark">
    <div class="container">
    {% block body %}
        <h1>Template error: body is not defined</h1>
    {% endblock %}
    </div>
    </body>
</html>
"#;

const NOT_FOUND: &str = r#"
{% extends "theme" %}
{% block head %}
<title>404</title>
{% endblock %}
{% block body %}
<div class="px-4 py-5 text-center">
    <h1 class="display-2 fw-bold">404</h1>
    {% block not_found_what %}
    <p>Your requested endpoint was not found.</p>
    {% endblock %}
    <a href="/" class="btn btn-primary">Return to home</a>
</div>
{% endblock %}
"#;

const SESSION_NOT_FOUND: &str = r#"
{% extends "notfound" %}
{% block not_found_what %}
<p>Could not find a session with this key: <code>{{ key }}</code></p>
{% endblock %}
"#;

const SESSION: &str = r#"
{% extends "theme" %}
{% block head %}
<title>Session</title>
{% endblock %}
{% block body %}
<h1>session content</h1>
{% endblock %}
"#;