use tokio::time::{sleep, Duration};

// Function to use grow tree
pub async fn grow_tree(height: i32,grow_factor: i32) {
    println!("Growing tree of height {}", height+grow_factor);
}

#[allow(dead_code)]
pub async fn main() {
    let  tree_height = 5;
    // let  grow_factor = 2;

    // Declare grow factor variable here
    let grow_factor = 6;
    
    // Call grow tree function
    grow_tree(tree_height,grow_factor).await;

    sleep(Duration::from_secs(3)).await;
    super::_level2::main().await; //super keyword will be working as same as self or this  it wraps all its members and methods
}