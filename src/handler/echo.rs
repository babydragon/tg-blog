use super::Handler;
use telegram_bot::types::{Message, MessageKind};
use telegram_bot::requests::SendMessage;
use telegram_bot::CanReplySendMessage;

pub struct EchoHandler {

}

impl Handler for EchoHandler {
    fn handle(&self, msg: Message) -> Option<SendMessage> {
        if let MessageKind::Text {ref data, ..} = msg.kind {
            let len = data.len();
            if len > 6 {
                let reply_text = &data[6 ..];
                return Some(msg.text_reply(reply_text.to_string()));
            }
            None      
        } else {
            None
        }
    }
}