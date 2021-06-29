mod iterator;
mod visitor;
pub use iterator::*;
pub use visitor::*;

pub struct Message {
    pub msg_id: u32,
    pub lang_id: u32,
    pub text: String,
}

impl Message {
    pub fn new(msg_id: u32, lang_id: u32, text: String) -> Self {
        Message {
            msg_id,
            lang_id,
            text,
        }
    }
}
