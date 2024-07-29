use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub price: f64
}

impl Item {

    pub fn new(name: String, price: f64) -> Self { 
        let id = Uuid::new_v4().to_string();
        Item {
            id,
            name, 
            price,
        }
    }

}
