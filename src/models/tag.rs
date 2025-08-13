#[derive(Debug, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,  // Store hex string or None if default
}

impl Tag {
    pub fn new(id: i64, name: String, color: Option<String>) -> Self {
        Self { id, name, color }
    }

    pub fn from_tuple(tuple: (i64, String, Option<String>)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}
