use diesel::Connection;
use diesel::pg::PgConnection;
use iron::prelude::*;
use iron;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct DatabaseMiddleware {
    pub db_conn: Arc<Mutex<PgConnection>>
}

impl DatabaseMiddleware {
    pub fn new(url: &str) -> DatabaseMiddleware {
        DatabaseMiddleware {
            db_conn: Arc::new(Mutex::new(PgConnection::establish(url).unwrap()))
        }
    }
}

impl iron::typemap::Key for DatabaseMiddleware {
    type Value = Arc<Mutex<PgConnection>>;
}

impl iron::middleware::BeforeMiddleware for DatabaseMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DatabaseMiddleware>(self.db_conn.clone());
        Ok(())
    }
}

pub trait DatabaseExt {
    fn db_conn(&self) -> MutexGuard<PgConnection>;
}

impl<'a, 'b> DatabaseExt for Request<'a, 'b> {
    fn db_conn(&self) -> MutexGuard<PgConnection> {
        let arc = self.extensions.get::<DatabaseMiddleware>().unwrap();
        arc.lock().unwrap()
    }
}
