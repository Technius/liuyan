use diesel;
#[allow(unused_imports)] use diesel::prelude::*; // maybe bug?

use models::*;

type Connection = diesel::pg::PgConnection;
type Res<T> = Result<T, diesel::result::Error>;

pub mod user {
    use super::*;
    use schema::users;

    pub fn register(username: String, conn: &Connection) -> Res<User> {
        let new_user = NewUser { username: username };
        diesel::insert(&new_user).into(users::table).get_result(conn)
    }

    pub fn find_by_id(id: i32, conn: &Connection) -> Res<User> {
        users::table.find(id).first(conn)
    }

    pub fn list(conn: &Connection) -> Res<Vec<User>> {
        users::table.load(conn)
    }
}

pub mod thread {
    use super::*;
    use schema::{comments, threads};

    pub fn create(slug: String, conn: &Connection) -> Res<Thread> {
        let new_thread = NewThread { slug: slug };
        diesel::insert(&new_thread).into(threads::table).get_result(conn)
    }

    pub fn find_by_id(id: i32, conn: &Connection) -> Res<Thread> {
        threads::table.find(id).first(conn)
    }

    pub fn list(conn: &Connection) -> Res<Vec<Thread>> {
        threads::table.load(conn)
    }

    pub fn list_comments(thread_id: i32, conn: &Connection) -> Res<Vec<Comment>> {
        comments::table
            .filter(comments::thread.eq(thread_id))
            .order(comments::id.desc())
            .load(conn)
    }

    pub fn comment_post(thread_id: i32, author: i32,
                        content: String, conn: &Connection) -> Res<Comment> {
        let new_comment = NewComment {
            thread: thread_id,
            author: author,
            content: content
        };
        diesel::insert(&new_comment).into(comments::table).get_result(conn)
    }
}
