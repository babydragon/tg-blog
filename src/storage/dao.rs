use rusqlite::{Connection, NO_PARAMS};
use rusqlite::types::*;
use super::{Blog, BlogKind};

static BLOG_TYPE_TEXT: &str = "text";
static BLOG_TYPE_IMG: &str = "img";

pub struct BlogDao {
    conn: Connection
}

impl BlogDao {
    pub fn new(db: String) -> BlogDao {
        let conn = Connection::open(db).expect("fail to open db");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS blog (
                create_time TEXT NOT NULL,
                type TEXT NOT NULL,
                text_content TEXT,
                img_content BLOB
            )", NO_PARAMS).expect("fail to init table");
        BlogDao {
            conn: conn
        }
    }

    pub fn insert(&self, blog: Blog) {
        let mut insert_stmt = self.conn.prepare_cached("INSERT INTO blog 
            (create_time, type, text_content, img_content)
            VALUES(?,?,?,?)").expect("fail to prepare insert");
        match blog.kind {
            BlogKind::Text(content) => {
                insert_stmt.insert(&[&blog.create_time as &ToSql, &BLOG_TYPE_TEXT, &content, &Null])
                    .expect("fail to insert blog");
            }

            BlogKind::Photo{ref caption, ref data} => {
                insert_stmt.insert(&[&blog.create_time as &ToSql, &BLOG_TYPE_IMG, &caption, &data])
                    .expect("fail to insert blog");
            }
        }
    }
}