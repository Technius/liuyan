use diesel::Connection;
use diesel::pg::PgConnection;
use iron::prelude::*;
use iron;
use iron::middleware;
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

impl middleware::BeforeMiddleware for DatabaseMiddleware {
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

/// Hack for properly clearing session cookies
pub struct DeleteCookieMiddleware;
impl middleware::AfterMiddleware for DeleteCookieMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        use iron::headers::SetCookie;
        {
            let headers = &mut res.headers;
            if let Some(sc) = headers.get_mut::<SetCookie>() {
                let SetCookie(ref mut cookies) = *sc;
                for c in cookies {
                    if c.starts_with("X-Liuyan-Session=; Max-Age=0;") {
                        c.push_str(";Path=/");
                    }
                }
            }
        }
        Ok(res)
    }
}
