use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub email: String,
}
