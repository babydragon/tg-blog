use telegram_bot::types::Message;
use telegram_bot::requests::SendMessage;

pub mod echo;
pub mod writer;
pub use self::echo::*;
pub use self::writer::*;

pub trait Handler {
    fn handle(&self, msg: Message) -> Option<SendMessage>;
}