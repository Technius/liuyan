extern crate iron;
extern crate router;
extern crate mount;
extern crate iron_json_response as ijr;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate config;

use iron::prelude::*;
use server_config::Config;
use ijr::JsonResponseMiddleware;

mod routes;
mod server_config;
mod schema;
mod models;
mod middleware;

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
    chain.link_before(middleware::SqliteMiddleware::new(config.db_url.as_str()));
    chain.link_after(JsonResponseMiddleware {});
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
