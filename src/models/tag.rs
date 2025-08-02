#[derive(Debug, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

impl Tag {
    pub fn new(id: i64, name: String) -> Self {
        Self { id, name }
    }

    pub fn from_tuple(tuple: (i64, String)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}
