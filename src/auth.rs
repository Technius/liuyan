use iron_sessionstorage;

pub struct SessionData {
    pub user_id: i32
}

impl iron_sessionstorage::Value for SessionData {
    fn get_key() -> &'static str { "liuyan-session" }

    fn into_raw(self) -> String { format!("{}", self.user_id) }

    fn from_raw(value: String) -> Option<SessionData> {
        value.as_str().parse::<i32>().map(|uid| SessionData { user_id: uid }).ok()
    }
}
