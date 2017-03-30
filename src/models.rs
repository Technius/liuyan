use ijr::JsonResponse;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Serialize)]
pub struct Thread {
    pub id: i32,
    pub slug: String,
}

#[derive(Queryable, Serialize)]
pub struct Comment {
    pub id: i32,
    pub author: i32,
    pub content: String,
}

#[derive(Serialize)]
pub enum ApiData {
    #[serde(rename = "threads")]
    Threads(Vec<Thread>)
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub data: ApiData
}

impl ApiResponse {
    pub fn new(data: ApiData) -> Self {
        ApiResponse { data: data }
    }

    pub fn json_response(data: ApiData) -> JsonResponse {
        JsonResponse::json(ApiResponse::new(data))
    }
}
