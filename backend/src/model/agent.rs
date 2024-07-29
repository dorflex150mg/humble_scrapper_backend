use uuid::Uuid;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
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

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
