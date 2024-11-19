use axum::{Router, routing::post, Json, http::StatusCode};
use axum::response::{IntoResponse};
use student::student_service::get_student_percentage;
use serde::Deserialize;

mod student;  // Import the student module

#[derive(Deserialize)]
pub struct StudentIdRequest {
    pub student_id: String,
    pub exam_name: String,
}

#[tokio::main]
async fn main() {
    // Create the Axum app with routes
    let app = Router::new()
        .route("/get_percentage", post(get_student_percentage));

    // Start the server on port 8080
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
