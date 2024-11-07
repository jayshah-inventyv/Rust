use tokio::time::{sleep, Duration};

pub async fn filling_tank(water_flow:&mut i32,max_limit:i32){
    for i in 1..=max_limit{
        println!("{}",water_flow);
        *water_flow+=1;
    }
    println!("The Tank is Full");
    sleep(Duration::from_secs(1)).await;
}

#[allow(dead_code)]
pub async fn main() {
    let mut water_flow=1;
    let max_limit=10;
    filling_tank(&mut water_flow,max_limit).await;
    super::_level3::main().await;
}