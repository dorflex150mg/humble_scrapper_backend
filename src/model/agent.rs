use uuid::Uuid;

pub struct Agent {
    pub id: String,
}

impl Agent {
    pub fn new() -> Self {
        Agent{
            id: Uuid::new_v4().to_string(),
        }
    }
}
