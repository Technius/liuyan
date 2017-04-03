use ijr::JsonResponse;
use schema::*;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Thread {
    pub id: i32,
    pub slug: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Comment {
    pub id: i32,
    pub thread: i32,
    pub author: i32,
    pub content: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String
}

#[derive(Insertable)]
#[table_name="threads"]
pub struct NewThread {
    pub slug: String
}

#[derive(Debug, Serialize)]
pub enum ApiData {
    #[serde(rename = "threads")]
    Threads(Vec<Thread>),
    #[serde(rename = "thread")]
    ThreadCreated(Thread),
    #[serde(rename = "users")]
    Users(Vec<User>),
    #[serde(rename = "user")]
    UserCreated(User),
    #[serde(rename = "user")]
    UserLoggedIn(User),
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub data: ApiData
}

impl ApiResponse {
    pub fn new(data: ApiData) -> Self {
        ApiResponse { data: data }
    }

    pub fn json(data: ApiData) -> JsonResponse {
        JsonResponse::json(ApiResponse::new(data))
    }
}
