use minijinja::Environment;
use serde::Deserialize;
use trust_dns_client::rr::RecordType;

use crate::session::OutputMode;

pub fn env() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_template("theme", THEME).unwrap();
    env.add_template("notfound", NOT_FOUND).unwrap();
    env.add_template("sessionnotfound", SESSION_NOT_FOUND)
        .unwrap();
    env.add_template("session", SESSION).unwrap();
    env.add_template("errorsaving", ERROR_SAVING).unwrap();
    env.add_template("noattempts", NO_ATTEMPTS).unwrap();
    env.add_template("home", HOME).unwrap();

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
<h1>Session</h1>
<p>Current output mode: {{ session.current_output_mode }}</p>

<div class="card my-3 border-info">
    <h5 class="card-header">IANA Authoritative Root Servers</h5>
    <ul class="d-flex flex-wrap card-body">
        <li class="badge rounded-pill text-bg-secondary m-1">198.41.0.4</li>
        <li class="badge rounded-pill text-bg-secondary m-1">199.9.14.201</li>
        <li class="badge rounded-pill text-bg-secondary m-1">192.33.4.12</li>
        <li class="badge rounded-pill text-bg-secondary m-1">199.7.91.13</li>
        <li class="badge rounded-pill text-bg-secondary m-1">192.203.230.10</li>
        <li class="badge rounded-pill text-bg-secondary m-1">192.5.5.241</li>
        <li class="badge rounded-pill text-bg-secondary m-1">192.112.36.4</li>
        <li class="badge rounded-pill text-bg-secondary m-1">198.97.190.53</li>
        <li class="badge rounded-pill text-bg-secondary m-1">192.36.148.17</li>
        <li class="badge rounded-pill text-bg-secondary m-1">192.58.128.30</li>
        <li class="badge rounded-pill text-bg-secondary m-1">193.0.14.129</li>
        <li class="badge rounded-pill text-bg-secondary m-1">199.7.83.42</li>
        <li class="badge rounded-pill text-bg-secondary m-1">202.12.27.33</li>
    </ul>
</div>

{% for item in session.user_requests %}
    {% if item.what.type == "Request" %}
    <div class="card my-3 {% if item.what.response.type != "Ok" %}border-danger{% else %}border-success{% endif %}">
        <h5 class="card-header">{{ item.when }}</h5>
        <div class="card-body">
            <ul class="list-group list-group-flush">
            <li class="list-group-item">
                Question: <code>IN {{item.what.request.record_type}} {{item.what.request.name}}</code> &rarr; <code>{{item.what.request.server_ip}}</code>
            </li>
            <li class="list-group-item">
                {% if item.what.response.type == "Ok" %}
                    Answer (output mode is <code>{{ item.what.response.resp.mode }}</code>):
                    <code><pre>{{ item.what.response.resp.text}}</pre></code>
                {% elif item.what.response.type == "QueryError" %}
                    Error while querying:
                    <code>{{item.what.response.err}}</code>
                {% elif item.what.response.type == "ForbiddenRecursion" %}
                    Querying this server is not allowed: <code>{{item.what.response.addr}}</code>
                {% elif item.what.response.type == "InvalidRequestIpAddr" %}
                    This is not a valid IP address: <code>{{item.what.response.addr}}</code>
                {% endif %}
            </li>
        </div>
    </div>
    {% elif item.what.type == "SwitchOutputMode" %}
        <div class="card my-3">
            <h5 class="card-header">{{ item.when }}</h5>
            <div class="card-body">
                <p>Switched output mode to <code>{{item.what.new_mode}}</code></p>
            </div>
        </div>
    {% elif item.what.type == "SubmitAnswer" %}
        <div class="card my-3 {% if item.what.status == "Correct" %}text-bg-success{% elif item.what.status == "Incorrect" %}text-bg-danger{% else %}border-warning{% endif %}">
            <h5 class="card-header">{{ item.when }}</h5>
            <div class="card-body">
                <p>Submitted answer: <code>{{item.what.answer}}</code> (which is {% if item.what.status == "Correct" %}correct{% elif item.what.status == "Incorrect" %}incorrect{% else %}of unknown validity due to an error, please try again{% endif %}).</p>
            </div>
        </div>

    {% endif %}
{% endfor %}

<hr>

<div class="card my-3 border-info">
    <h5 class="card-header">Question</h5>
    <p class="card-body">
        {{session.question.text}}
    </p>
</div>


<hr>
<div class="{% if session.can_answer %}{% else %}noanswer{% endif %}">
    <div id="new-query" class="my-3">
        <h2>New query:</h2>
        <form method=POST>
            <div class="input-group">
            <span class="input-group-text"><code>dig @</code></span>
            <input class="form-control" type=text name="ip" placeholder="NS server IP" style="flex: 5;" {% if session.can_answer %}{% else %}disabled{% endif %}/>
            <span class="input-group-text"><code> IN </code></span>
            <select name="class" class="form-control" style="flex: 1;" {% if session.can_answer %}{% else %}disabled{% endif %}>
                <option value="A" selected>A</option>
                <option value="AAAA">AAAA</option>
                <option value="ANAME">ANAME</option>
                <option value="AXFR">AXFR</option>
                <option value="CAA">CAA</option>
                <option value="CDS">CDS</option>
                <option value="CDNSKEY">CDNSKEY</option>
                <option value="CNAME">CNAME</option>
                <option value="CSYNC">CSYNC</option>
                <option value="DNSKEY">DNSKEY</option>
                <option value="DS">DS</option>
                <option value="HINFO">HINFO</option>
                <option value="HTTPS">HTTPS</option>
                <option value="IXFR">IXFR</option>
                <option value="KEY">KEY</option>
                <option value="MX">MX</option>
                <option value="NAPTR">NAPTR</option>
                <option value="NS">NS</option>
                <option value="NSEC">NSEC</option>
                <option value="NSEC3">NSEC3</option>
                <option value="NSEC3PARAM">NSEC3PARAM</option>
                <option value="NULL">NULL</option>
                <option value="OPENPGPKEY">OPENPGPKEY</option>
                <option value="OPT">OPT</option>
                <option value="PTR">PTR</option>
                <option value="RRSIG">RRSIG</option>
                <option value="SIG">SIG</option>
                <option value="SOA">SOA</option>
                <option value="SRV">SRV</option>
                <option value="SSHFP">SSHFP</option>
                <option value="SVCB">SVCB</option>
                <option value="TLSA">TLSA</option>
                <option value="TSIG">TSIG</option>
                <option value="TXT">TXT</option>
            </select>
            <input type=text class="form-control" name="name" placeholder="Domain name" style="flex: 5;" {% if session.can_answer %}{% else %}disabled{% endif %}/>
            <input type=hidden name="action" value="Query" />
            <input type=submit  class="btn btn-outline-success" value="Go!" {% if session.can_answer %}{% else %}disabled{% endif %} />
        </form>
    </div>
    <div class="my-3">
        <form method=POST>
        <div class="input-group">
        <span class="input-group-text">New output mode:</span>
        <select name="mode" class="form-control" style="flex: 1;" {% if session.can_answer %}{% else %}disabled{% endif %}>
            <option value="Classic">Classic</option>
            <option value="Rust">Rust</option>
        </select>
        <input type=hidden name="action" value="SetOutputMode" />
        <input type=submit  class="btn btn-outline-success" value="Go!" {% if session.can_answer %}{% else %}disabled{% endif %} />
        </form>
    </div>
    <div class="my-3">
        <form method=POST>
        <div class="input-group">
        <span class="input-group-text">Submit an answer:</span>
        <span class="input-group-text text-bg-warning">({{ session.answers_remaining }} attempts left)</span>
        <input type=text class="form-control" name="answer" style="flex: 5;" {% if session.can_answer %}{% else %}disabled{% endif %}/>

        <input type=hidden name="action" value="SubmitAnswer"/>
        <input type=submit  class="btn btn-danger" value="{% if session.can_answer %}Submit{% else %}Cannot submit{% endif %}!" {% if session.can_answer %}{% else %}disabled{% endif %}/>
        </form>
    </div>
    <div class="noanswer-stroke">
    {% if session.can_answer %}{% else %}
        <h1></h1>
        <h1 class="text-center" style="margin: auto;">
        Cannot submit query at this time.
        </h1>
    {% endif %}
    </div>
</div>
<style>
    .noanswer {
        position: relative;
        display: block;
    }
    .noanswer .noanswer-stroke {
        /* https://stackoverflow.com/a/22565186/5936187, https://css-tricks.com/stripes-css/ */
        background-image: 
        repeating-linear-gradient(
            45deg,
            rgba(0, 0, 0, 0.5),
            rgba(0, 0, 0, 0.5) 10px,
            rgba(0, 0, 0, 0.5) 10px,
            rgba(0, 0, 0, 0.5) 50px
          );
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
        border-style: solid;
        border-color: white;
        border-radius: 16px;
        display: flex;
        flex-direction: column;
    }
</style>
{% endblock %}
"#;

#[derive(Deserialize, Debug)]
#[serde(tag = "action")]
pub enum SessionRequest {
    Query {
        ip: String,
        class: RecordType,
        name: String,
    },
    SetOutputMode {
        mode: OutputMode,
    },
    SubmitAnswer {
        answer: String,
    },
}

const ERROR_SAVING: &str = r#"
{% extends "theme" %}
{% block head %}
<title>Server error</title>
{% endblock %}
{% block body %}
<div class="px-4 py-5 text-center">
    <h1 class="display-2 fw-bold">Internal error</h1>
    <p>There was an error while saving your session. Your latest action has not been saved.</p>
    <a href="/{{ key }}?aftererror=1" class="btn btn-primary">Return to your session</a>
</div>
{% endblock %}
"#;

const NO_ATTEMPTS: &str = r#"
{% extends "theme" %}
{% block head %}
<title>No attempts left</title>
{% endblock %}
{% block body %}
<div class="px-4 py-5 text-center">
    <h1 class="display-2 fw-bold">No attempts left</h1>
    <p>You have exceeded your attempt limit. Your latest action has not been saved.</p>
    <a href="/{{ key }}?aftererror=1" class="btn btn-primary">Return to your session</a>
</div>
{% endblock %}
"#;


const HOME: &str = r#"
{% extends "theme" %}
{% block head %}
<title>DNS</title>
{% endblock %}
{% block body %}
<div class="px-4 py-5 text-center">
    <h1 class="display-2 fw-bold">DNS</h1>
    <p>Please enter your session key to begin your task.</p>
    <form method=POST>
        <div class="input-group">
        <span class="input-group-text">Session key:</span>
        <input type=text class="form-control" name="key" style="flex: 5;"/>
        <input type=submit class="btn btn-outline-success" value="Go!"/>
        </div>
    </form>

</div>
{% endblock %}
"#;

#[derive(Deserialize, Debug)]
pub struct SessionKeyRequest {
    pub key: String,
}