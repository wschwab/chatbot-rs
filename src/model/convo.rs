use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Convo {
    pub msgs: Vec<Message>
}

impl Convo {
    pub fn new() -> Convo {
        Convo {
            msgs: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,
    pub text: String,
}