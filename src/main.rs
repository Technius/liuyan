#[macro_use]
extern crate iron;
extern crate router;
extern crate mount;
extern crate params;
extern crate iron_sessionstorage;
extern crate iron_json_response as ijr;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate config;
extern crate time;
extern crate base64;

use iron::prelude::*;
use server_config::Config;
use ijr::JsonResponseMiddleware;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

mod routes;
mod server_config;
mod schema;
mod models;
mod repo;
mod middleware;
mod auth;

fn main() {
    match Config::load("application.yaml") {
        Ok(config) => start_server(&config),
        Err(e) => {
            println!("Error while starting server:");
            println!("{}", e);
        }
    }
}

fn start_server(config: &Config) {
    let mount = routes::create();
    let url = format!("{}:{}", config.host, config.port);
    let mut chain = Chain::new(mount);
    chain.link_before(middleware::DatabaseMiddleware::new(config.db_url.as_str()));
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(config.secret.clone())));
    chain.link_after(JsonResponseMiddleware {});
    chain.link_after(middleware::DeleteCookieMiddleware {});
    let start_status = Iron::new(chain).http(&url);
    match start_status {
        Ok(_) => {
            println!("Server started on {}", &url);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
