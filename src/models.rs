#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CurrentPage {
    Tasks,
    Archives,
}

#[derive(Debug, Default)]
pub struct TodoItem {
    pub is_done: bool,
    pub description: String,
}

impl TodoItem {
    pub fn new(description: &str) -> Self {
        Self {
            is_done: false,
            description: description.to_string(),
        }
    }
}