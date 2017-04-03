use iron::prelude::*;
use iron::status;
use iron_sessionstorage::SessionRequestExt;
use router::Router;
use mount::Mount;
use diesel;
use diesel::associations::HasTable;
use diesel::prelude::*;
use params;
use params::FromValue;

use auth::SessionData;
use middleware::DatabaseExt;
use models::*;

macro_rules! require_login {
    ($req:ident) => {
        match $req.session().get::<SessionData>().unwrap() {
            Some(user) => user,
            None => return Ok(Response::with((status::Forbidden, "Login required")))
        }
    }
}

fn get_param<T: FromValue>(name: &str, req: &mut Request) -> Option<T> {
    req.get_ref::<params::Params>().unwrap()
        .find(&[name])
        .and_then(|v| T::from_value(v))
}

pub fn create() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/threads", thread());
    mount.mount("/users", user());
    mount
}

fn thread() -> Router {
    use schema::threads::dsl::*;
    let mut router = Router::new();

    router.get("/", |req: &mut Request| {
        let xs = itry!(threads.load::<Thread>(&*req.db_conn()));
        let response = ApiResponse::json(ApiData::Threads(xs));
        Ok(Response::with((status::Ok, response)))
    }, "thread_list");

    router.post("/", |req: &mut Request| {
        require_login!(req);
        let p_slug = iexpect!(get_param("slug", req), (status::BadRequest, "missing slug"));
        let t = diesel::insert(&NewThread { slug: p_slug })
            .into(threads::table())
            .get_result(&*req.db_conn())
            .expect("Failed to create thread");
        let res = ApiResponse::json(ApiData::ThreadCreated(t));
        Ok(Response::with((status::Created, res)))
    }, "thread_create");

    router
}

fn user() -> Router {
    use schema::users::dsl::*;
    let mut router = Router::new();

    router.get("/", |req: &mut Request| {
        let c = req.db_conn();
        let xs = itry!(users.load::<User>(&*c));
        let response = ApiResponse::json(ApiData::Users(xs));
        Ok(Response::with((status::Ok, response)))
    }, "user_list");

    router.get("/testLogin", |req: &mut Request| {
        let sd = require_login!(req);
        Ok(Response::with((status::Ok, format!("id: {}", sd.user_id))))
    }, "user_testlogin");

    router.get("/register", |req: &mut Request| {
        req.session().clear().expect("Failed to clear session");
        let name = get_param::<String>("username", req);
        let name = iexpect!(name, (status::BadRequest, "missing username parameter"));
        let new_user = NewUser { username: name };
        let user = diesel::insert(&new_user)
            .into(users::table())
            .get_result::<User>(&*req.db_conn())
            .expect("Failed to create user");
        req.session().set(SessionData::new(user.id)).unwrap();
        let response = ApiResponse::json(ApiData::UserCreated(user));
        Ok(Response::with((status::Created, response)))
    }, "user_register");

    router.get("/login", |req: &mut Request| {
        let uid = iexpect!(get_param::<i32>("id", req), (status::BadRequest, "missing id"));
        let user = itry!(users.find(uid).first::<User>(&*req.db_conn()), status::NotFound);
        req.session().set(SessionData::new(uid)).unwrap();
        let response = ApiResponse::json(ApiData::UserLoggedIn(user));
        Ok(Response::with((status::Ok, response)))
    }, "user_login");

    router.get("/logout", |req: &mut Request| {
        require_login!(req);
        try!(req.session().clear());
        Ok(Response::with((status::Ok, "logged out")))
    }, "user_logout");

    router
}
