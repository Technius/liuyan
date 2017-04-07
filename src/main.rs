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
extern crate r2d2;
extern crate r2d2_diesel;
extern crate config;
extern crate time;
extern crate base64;

use diesel::pg::PgConnection;
use ijr::JsonResponseMiddleware;
use iron::prelude::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use server_config::Config;

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
    let url = format!("{}:{}", config.host, config.port);
    let db_config = r2d2::Config::default();
    let db_manager = ConnectionManager::<PgConnection>::new(config.db_url.as_str());
    let db_pool = Pool::new(db_config, db_manager).expect("Failed to connect to database");

    let mount = routes::create();
    let mut chain = Chain::new(mount);
    chain.link_before(middleware::DatabaseMiddleware::new(db_pool));
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
