use std::env;

extern crate actix_web;

use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello World 2"
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> String {
    env::var("PORT").ok().unwrap_or("8080".to_string())
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(format!("{}:{}", "127.0.0.1", get_server_port()))
        .unwrap()
        .run();
}
