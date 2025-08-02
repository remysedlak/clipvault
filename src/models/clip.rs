#[derive(Debug, Clone)]
pub struct Clip {
    pub id: i64,
    pub content: String,
    pub timestamp: i64,
    pub pinned: bool,
}

impl Clip {
    pub fn new(id: i64, content: String, timestamp: i64, pinned: bool) -> Self {
        Self { id, content, timestamp, pinned }
    }

    pub fn from_tuple(tuple: (i64, String, i64, bool)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}