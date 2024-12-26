use uuid::Uuid;

pub trait ChattyDataTag {
    fn tag(&self) -> Uuid;
}

pub struct ChattyDataHandler {}
impl ChattyDataHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl ChattyDataHandler {
    pub fn handler(&self, _data: &String) {}
}
