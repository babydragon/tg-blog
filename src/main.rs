extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate chrono;
extern crate rusqlite;
extern crate curl;

mod handler;
mod storage;
mod router;

use self::router::Router;
use self::handler::*;
use self::storage::BlogDao;
use std::env;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use std::rc::Rc;

fn main() {
    let db = "blog.db".to_string();
    let mut core = Core::new().expect("fail to init tokio core");
    let handle = core.handle();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("fail to get token from env");
    let api = Api::configure(&token).build(core.handle()).expect("fail to init tg api");
    let mut router = Router::new();
    let blog_dao = Rc::new(BlogDao::new(db));

    let echo_handler = EchoHandler{};
    router.add_text_hander("/echo".to_string(), Box::new(echo_handler));

    let text_writer_handler = TextWriterHandler::new(blog_dao.clone());
    router.set_text_handler(Box::new(text_writer_handler));

    let photo_writer_handler = PhotoWriterHandler::new(blog_dao.clone(), api.clone(), handle.clone(), token);
    router.set_photo_handler(Box::new(photo_writer_handler));

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            if let Some(req) = router.handle(message) {
                api.spawn(req);
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
