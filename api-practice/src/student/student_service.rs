use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::student::student_structure::{SaveAcknowledgeRequest, Student, Marks};
use serde_json::json;
use std::fs;

pub async fn save_acknowledgement(
    save_ack_req: Result<Json<SaveAcknowledgeRequest>, axum::extract::JsonRejection>
) -> impl IntoResponse {
    match save_ack_req {
        Ok(save_ack_req) => match save_acknowledgement_in_db(save_ack_req.0).await {
            Ok(data) => {
                // Return success response
                (StatusCode::OK, Json(Message { 
                    status: 2000, 
                    message_key: "Acknowledgement saved successfully", 
                    data 
                })).into_response()
            }
            Err(err) => {
                // Return internal server error
                (StatusCode::INTERNAL_SERVER_ERROR, Json(Message { 
                    status: 5002, 
                    message_key: "Failed to save acknowledgement", 
                    data: err 
                })).into_response()
            }
        },
        Err(err) => {
            // Return bad request error if validation fails
            (StatusCode::BAD_REQUEST, Json(Message { 
                status: 4002, 
                message_key: "Invalid request", 
                data: err.to_string() 
            })).into_response()
        }
    }
}

// Helper function to simulate saving an acknowledgment
async fn save_acknowledgement_in_db(req: SaveAcknowledgeRequest) -> Result<String, String> {
    // Simulate saving data (in reality, you may interact with a database or file)
    let students = read_students_data();
    if let Some(student) = students.iter().find(|s| s.id == req.student_id) {
        // Simulate saving the acknowledgment
        return Ok(format!("Acknowledgement saved for student: {}", student.name));
    }
    
    Err("Student not found".to_string())
}

// Simulate reading student data from a JSON file (you could replace this with actual DB logic)
fn read_students_data() -> Vec<Student> {
    let file_content = fs::read_to_string("studentJson.json").expect("Error reading file");
    serde_json::from_str(&file_content).expect("Error parsing JSON")
}

// Message structure used for API responses
#[derive(Serialize, Deserialize)]
struct Message {
    pub status: i32,
    pub message_key: &'static str,
    pub data: String,
}
