use tokio::time::{sleep, Duration};

pub async fn jumb_mountain(path:String){
    println!("Climbing the mountain: {}",path);
}

pub async  fn main(){
    let path =String::from("Intial Steps towards Mountains");
    println!("path {}",path);
    jumb_mountain(path).await;

    // Uncommenting the line below would cause a compilation error
    // println!("Back on the {}.", path); // Error: value borrowed here after move HERE OWNER IS TRANSFER

    sleep(Duration::from_secs(3)).await;


    // Call the next level
    super::_level4::main().await;
}