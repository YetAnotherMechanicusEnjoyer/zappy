#[derive(Debug, Clone)]
pub struct Team {
    pub name: String,
}

impl Team {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
