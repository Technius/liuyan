use iron::prelude::*;
use iron::status;
use iron_sessionstorage::SessionRequestExt;
use router::Router;
use mount::Mount;
use params;
use params::FromValue;
use std::str::FromStr;

use auth::SessionData;
use middleware::DatabaseExt;
use models::*;
use repo;

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

fn get_segment<T: FromStr>(name: &str, req: &Request) -> Result<T, T::Err> {
    req.extensions.get::<Router>().unwrap()[name].parse::<T>()
}

pub fn create() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/threads", thread());
    mount.mount("/users", user());
    mount
}

fn thread() -> Router {
    let mut router = Router::new();

    router.get("/", |req: &mut Request| {
        let xs = itry!(repo::thread::list(&req.db_conn()));
        let response = ApiResponse::json(ApiData::Threads(xs));
        Ok(Response::with((status::Ok, response)))
    }, "thread_list");

    router.post("/", |req: &mut Request| {
        require_login!(req);
        let p_slug = iexpect!(get_param("slug", req), (status::BadRequest, "missing slug"));
        let t = itry!(repo::thread::create(p_slug, req.db_conn()));
        let res = ApiResponse::json(ApiData::ThreadCreated(t));
        Ok(Response::with((status::Created, res)))
    }, "thread_create");

    router.get("/:id", |req: &mut Request| {
        let db = req.db_conn();
        let thread_id = itry!(get_segment::<i32>("id", req), status::NotFound);
        let thread = itry!(repo::thread::find_by_id(thread_id, db), status::NotFound);
        let comments = itry!(repo::thread::list_comments(thread_id, db));
        let response = ApiResponse::json(ApiData::ThreadShow { thread: thread, comments: comments });
        Ok(Response::with((status::Ok, response)))
    }, "thread_show");

    router.post("/:id", |req: &mut Request| {
        let session = require_login!(req);
        let thread_id = itry!(get_segment::<i32>("id", req), status::NotFound);
        let content = iexpect!(get_param::<String>("content", req), (status::BadRequest, "missing content"));

        if content.is_empty() {
            return Ok(Response::with((status::BadRequest, "content is blank")));
        }

        let db = req.db_conn();
        // Check if thread exists, first
        let _ = itry!(repo::thread::find_by_id(thread_id, db), status::NotFound);
        let comment = itry!(repo::thread::comment_post(thread_id, session.user_id, content, db));
        let response = ApiResponse::json(ApiData::CommentPost(comment));
        Ok(Response::with((status::Ok, response)))
    }, "thread_comment_post");

    router
}

fn user() -> Router {
    let mut router = Router::new();

    router.get("/", |req: &mut Request| {
        let xs = itry!(repo::user::list(req.db_conn()));
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
        let user = itry!(repo::user::register(name, req.db_conn()));
        req.session().set(SessionData::new(user.id)).unwrap();
        let response = ApiResponse::json(ApiData::UserCreated(user));
        Ok(Response::with((status::Created, response)))
    }, "user_register");

    router.get("/login", |req: &mut Request| {
        let uid = iexpect!(get_param::<i32>("id", req), (status::BadRequest, "missing id"));
        let user = itry!(repo::user::find_by_id(uid, req.db_conn()), status::NotFound);
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
