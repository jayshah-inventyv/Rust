use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub marks: HashMap<String, Marks>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Students {
    pub students: Vec<Student>,
}
