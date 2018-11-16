pub mod dao;
pub use self::dao::BlogDao;

use chrono::NaiveDateTime;

pub enum BlogKind {
    Text(String),
    Photo {
        caption: Option<String>,
        data: Vec<u8>
    }
}

pub struct Blog {
    pub create_time: NaiveDateTime,
    pub kind: BlogKind
}