use serde_json::Value;

/// Represents a page of data in paginated results
#[derive(Debug, Clone)]
pub struct Page {
    pub data: Vec<Value>,
    pub next_token: Option<String>,
    pub page_number: Option<usize>,
}

impl Page {
    pub fn new(data: Vec<Value>) -> Self {
        Self {
            data,
            next_token: None,
            page_number: None,
        }
    }

    pub fn with_next_token(mut self, token: String) -> Self {
        self.next_token = Some(token);
        self
    }

    pub fn with_page_number(mut self, page: usize) -> Self {
        self.page_number = Some(page);
        self
    }
}
