use tokio::time::{sleep,Duration};

pub async fn fix_machine(parts:u16) -> Result<String,String>{
    if parts >= 3{
        Ok("The machine has been succesfully repaired!".to_string())
    }else{
        Err("Not enough parts to repair the machine. At least 3 parts are required.".to_string())
    }
}

pub async fn main(){
    let parts_available = 4; // Change this to test success or failure scenarios

    println!("Entering the Dark Mines of Errors...");
    sleep(Duration::from_secs(2)).await;

    // Attempt to fix the machine and handle the result using match
    match fix_machine(parts_available).await {
        Ok(success_message) => println!("{}", success_message),
        Err(error_message) => println!("Error: {}", error_message),
    }

    // Async pause for pacing
    println!("Taking a moment after working on the machine...");
    sleep(Duration::from_secs(3)).await;

    // Call the next level
    super::_level5::main().await;
}