use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub age: u8,
    pub department: String,
}

impl Student {
    pub fn new(name: String, age: u8, department: String) -> Student {
        Student {
            id: Uuid::new_v4(),
            name,
            age,
            department,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateStudent {
    pub name: String,
    pub age: u8,
    pub department: String,
}
