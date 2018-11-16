use super::Handler;
use crate::storage::*;
use telegram_bot::types::{Message, MessageKind, PhotoSize};
use telegram_bot::types::requests::*;
use telegram_bot::requests::SendMessage;
use telegram_bot::CanReplySendMessage;
use telegram_bot::Api;
use futures::Future;
use chrono::prelude::*;
use std::rc::Rc;
use tokio_core::reactor::Handle;
use curl::easy::Easy;

pub struct TextWriterHandler {
    dao: Rc<BlogDao>
}

impl TextWriterHandler {
    pub fn new(dao: Rc<BlogDao>) -> TextWriterHandler {
        TextWriterHandler {
            dao: dao
        }
    }
}

impl Handler for TextWriterHandler {
    fn handle(&self, msg: Message) -> Option<SendMessage> {
        if let MessageKind::Text {ref data, ..} = msg.kind {
            let blog_kind = BlogKind::Text(data.to_string());
            let blog = Blog {
                create_time: Local::now().naive_local(),
                kind: blog_kind
            };
            self.dao.insert(blog);
            return Some(msg.text_reply("记录成功"));
        } else {
            None
        }
    }
}

pub struct PhotoWriterHandler {
    dao: Rc<BlogDao>,
    api: Api,
    handle: Handle,
    token: String
}

impl PhotoWriterHandler {
    pub fn new(dao: Rc<BlogDao>, api: Api, handle: Handle, token: String) -> PhotoWriterHandler {
        PhotoWriterHandler {
            dao: dao,
            api: api,
            handle: handle,
            token: token
        }
    }
}

impl Handler for PhotoWriterHandler {
    fn handle(&self, msg: Message) -> Option<SendMessage> {
        if let MessageKind::Photo{data, caption} = msg.kind {
            let empty_size = PhotoSize {
                file_id: "".to_string(),
                width: 0,
                height: 0,
                file_size: None
            };
            let max_size_photo = data.iter().fold(empty_size, |max, item| {
                let max_size = max.width * max.height;
                let current = item.width * item.height;
                if current > max_size {
                    let result = item.clone();
                    return result;
                }

                max
            });

            let get_file = GetFile::new(max_size_photo);
            let blog_dao = self.dao.clone();
            let token = self.token.clone();
            let future = self.api.send(get_file).and_then(move|file| {
                if let Some(url) = file.get_url(token.as_str()) {
                    let mut buf : Vec<u8> = Vec::new();
                    let mut easy = Easy::new();
                    easy.url(&url).unwrap();
                    {
                        let mut transfer = easy.transfer();
                        transfer.write_function(|data| {
                            buf.extend_from_slice(data);
                            Ok(data.len())
                        }).unwrap();
                        transfer.perform().unwrap();
                    }
                    let blog_kind = BlogKind::Photo {
                        caption: caption,
                        data: buf
                    };
                    let blog = Blog {
                        create_time: Local::now().naive_local(),
                        kind: blog_kind
                    };
                    blog_dao.insert(blog);
                }
                Ok(())
            });

            self.handle.spawn(future.then(|_| Ok(())));
        }

        None
    }
}