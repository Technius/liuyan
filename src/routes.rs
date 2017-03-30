use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use diesel::prelude::*;

use models::*;
use middleware::SqlExt;

pub fn create() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/", thread());
    mount
}

fn thread() -> Router {
    let mut router = Router::new();
    router.get("/",
               |req: &mut Request| {
                   let c = req.db_conn();
                   use schema::threads::dsl::*;
                   let xs = threads.load::<Thread>(&*c).expect("Error loading threads");
                   let response = ApiResponse::json_response(ApiData::Threads(xs));
                   Ok(Response::with((status::Ok, response)))
               },
               "dir");
    router
}
