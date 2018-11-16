use telegram_bot::types::{Message, MessageKind};
use telegram_bot::requests::SendMessage;
use super::handler::Handler;
use std::collections::HashMap;

pub struct Router {
    text_handler_map: HashMap<String, Box<Handler>>,
    photo_handler: Option<Box<Handler>>,
    text_handler: Option<Box<Handler>>
}

impl Router {
    pub fn new() -> Router {
        Router {
            text_handler_map: HashMap::new(),
            photo_handler: None,
            text_handler: None
        }
    }

    pub fn add_text_hander(&mut self, command: String, handler: Box<Handler>) {
        self.text_handler_map.insert(command, handler);
    }

    pub fn set_photo_handler(&mut self, h: Box<Handler>) {
        self.photo_handler = Some(h);
    }

    pub fn set_text_handler(&mut self, h: Box<Handler>) {
        self.text_handler = Some(h);
    }

    pub fn handle(&mut self, msg: Message) -> Option<SendMessage> {
        match msg.kind {
            MessageKind::Text {ref data, ..} => {
                if data.starts_with("/") {
                    if let Some(index) = data.find(" ") {
                        let command: String = data.chars().take(index).collect();
                        if let Some(handler) = self.text_handler_map.get(&command) {
                            return handler.handle(msg);
                        }
                    }
                } else if let Some(handler) = &self.text_handler {
                    return handler.handle(msg);
                }
            }

            MessageKind::Photo{..} => {
                if let Some(handle) = &self.photo_handler {
                    return handle.handle(msg);
                }
            }

            MessageKind::Document{..} => {
                if let Some(handle) = &self.photo_handler {
                    return handle.handle(msg);
                }
            }

            _ => {
                // ignore
            }
        }

        None
    }
}