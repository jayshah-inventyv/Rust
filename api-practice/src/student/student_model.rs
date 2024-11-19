use crate::student::student_structure::Student;
use serde_json::{Value, json};
use std::{fs, collections::HashMap};

// Reads student data from a JSON file
pub fn read_students_data() -> Vec<Student> {
    let file_content = fs::read_to_string("studentJson.json").expect("Error reading file");
    serde_json::from_str(&file_content).expect("Error parsing JSON")
}

// Writes student data back to the JSON file
pub fn write_students_data(students: &Vec<Student>) {
    let updated_data = serde_json::to_string_pretty(students).expect("Error serializing data");
    fs::write("studentJson.json", updated_data).expect("Error writing to file")
}
