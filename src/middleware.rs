use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use iron::prelude::*;
use iron;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct SqliteMiddleware {
    pub db_conn: Arc<Mutex<SqliteConnection>>
}

impl SqliteMiddleware {
    pub fn new(url: &str) -> SqliteMiddleware {
        SqliteMiddleware {
            db_conn: Arc::new(Mutex::new(SqliteConnection::establish(url).unwrap()))
        }
    }
}

impl iron::typemap::Key for SqliteMiddleware {
    type Value = Arc<Mutex<SqliteConnection>>;
}

impl iron::middleware::BeforeMiddleware for SqliteMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<SqliteMiddleware>(self.db_conn.clone());
        Ok(())
    }
}

pub trait SqlExt {
    fn db_conn(&self) -> MutexGuard<SqliteConnection>;
}

impl<'a, 'b> SqlExt for Request<'a, 'b> {
    fn db_conn(&self) -> MutexGuard<SqliteConnection> {
        let arc = self.extensions.get::<SqliteMiddleware>().unwrap();
        arc.lock().unwrap()
    }
}
