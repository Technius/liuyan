use diesel::pg::PgConnection;
use iron::prelude::*;
use iron;
use iron::middleware;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

type DBPool = Pool<ConnectionManager<PgConnection>>;

pub struct DatabaseMiddleware {
    pub pool: DBPool
}

impl DatabaseMiddleware {
    pub fn new(pool: DBPool) -> DatabaseMiddleware {
        DatabaseMiddleware {
            pool: pool
        }
    }
}

impl iron::typemap::Key for DatabaseMiddleware {
    type Value = PooledConnection<ConnectionManager<PgConnection>>;
}

impl middleware::BeforeMiddleware for DatabaseMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DatabaseMiddleware>(self.pool.get().unwrap());
        Ok(())
    }
}

pub trait DatabaseExt {
    fn db_conn(&self) -> &PgConnection;
}

impl<'a, 'b> DatabaseExt for Request<'a, 'b> {
    fn db_conn(&self) -> &PgConnection {
        self.extensions.get::<DatabaseMiddleware>().unwrap()
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

/// Middleware for inserting Access-Control-Allow-Origin header
pub struct CorsMiddleware {
    domain: String
}
impl CorsMiddleware {
    pub fn new(domain: &String) -> CorsMiddleware {
        CorsMiddleware {
            domain: domain.clone()
        }
    }
}
impl middleware::AfterMiddleware for CorsMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        use iron::headers::AccessControlAllowOrigin;
        res.headers.set(AccessControlAllowOrigin::Value(self.domain.clone()));
        Ok(res)
    }
}
