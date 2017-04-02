use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use diesel::prelude::*;
use params;
use params::FromValue;

use models::*;
use middleware::SqlExt;
use iron_sessionstorage::SessionRequestExt;

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
    mount.mount("/", thread());
    mount.mount("/users", user());
    mount
}

fn thread() -> Router {
    let mut router = Router::new();
    use schema::threads::dsl::*;
    router.get("/", |req: &mut Request| {
        let c = req.db_conn();
        let xs = itry!(threads.load::<Thread>(&*c));
        let response = ApiResponse::json_response(ApiData::Threads(xs));
        Ok(Response::with((status::Ok, response)))
    }, "dir");
    router
}

fn user() -> Router {
    let mut router = Router::new();
    use schema::users::dsl::*;
    router.get("/", |req: &mut Request| {
        let c = req.db_conn();
        let xs = itry!(users.load::<User>(&*c));
        let response = ApiResponse::json_response(ApiData::Users(xs));
        Ok(Response::with((status::Ok, response)))
    }, "dir");
    router.get("/register", |req: &mut Request| {
        req.session().clear().expect("Failed to clear session");
        let name = get_param::<String>("username", req);
        let name = iexpect!(name, (status::BadRequest, "missing username parameter"));
        // TODO: Fix this. sqlite is incompatible with get_result
        // let c = req.db_conn();
        // let new_user = NewUser { username: name };
        // let user = diesel::insert(&new_user)
        //     .into(users::table())
        //     .get_result(&*c)
        //     .expect("Failed to create user");
        // let response = ApiResponse::json_response(ApiData::UserCreated(user));
        // Ok(Response::with((status::Created, response)))
        Ok(Response::with((status::Ok, "Todo")))
    }, "register");
    router.get("/login", |req: &mut Request| {
        Ok(Response::with((status::Ok, "foo")))
    }, "login");
    router.get("/logout", |req: &mut Request| {
        req.session().clear().expect("Failed to clear session");
        Ok(Response::with((status::Ok, "logged out")))
    }, "logout");
    router
}
