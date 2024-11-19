// use get;
use std::net::SocketAddr;
pub mod configration;

#[tokio::main]
async fn main() {
    // Create the Axum router
    // let app = Router::new().route("/get_percentage", post(get_student_percentage));

    // Run the Axum server on localhost:8080
    // match configration::initialize_config().await {
        // Ok(_) => {
            let app = get_routes();
            let addr = SocketAddr::from(([0, 0, 0, 0],4));
            // println!("Started Server On Port {:?}", configration::get::<i64>("port"));
            println!("Student API Strated");

            axum::Server::bind(&addr).serve(app.clone().into_make_service()).await.expect("something went wrong!");
        // }
    //     Err(err) => {
    //         println!("Error while initialize config{:?}", err);
    //     }
    // }

}
