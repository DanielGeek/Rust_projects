use super::{message::Message, options::ChatRequestOptions};

pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub raw: bool,
    pub tools: (),
    pub options: Option<ChatRequestOptions>,
}
