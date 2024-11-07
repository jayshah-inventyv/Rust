use tokio::time::{sleep, Duration};

// Define the CreatureType enum to classify the creature's type
enum CreatureType {
    Flying,
    Ground,
}

// Define the Creature struct to represent a creature with name, health, and type
struct Creature {
    name: String,
    health: u32,
    creature_type: CreatureType,
}

// Function to describe the creature based on its struct and enum type
fn describe_creature(creature: &Creature) {
    println!("Encountered a creature named: {}", creature.name);
    println!("Health: {}", creature.health);

    // Match on creature type to display specific behavior
    match creature.creature_type {
        CreatureType::Flying => println!("This creature soars through the sky!"),
        CreatureType::Ground => println!("This creature roams the land."),
    }
}

// Main function for Level 5, introducing structs and enums
pub async fn main() {
    println!("Welcome to the Fields of Structs and Enums!");

    // Simulate a creature with name, health, and type
    let creature = Creature {
        name: "Mystic Griffin".to_string(),
        health: 80,
        creature_type: CreatureType::Flying,
    };

    // Describe the creature using the function
    describe_creature(&creature);

    // Async pause to add pacing
    println!("Preparing for the next encounter...");
    sleep(Duration::from_secs(2)).await;

    println!("You have successfully explored the Fields of Structs and Enums!");

    // Pause before transitioning to the next level
    sleep(Duration::from_secs(3)).await;
    
    // Move to the next level, if available
    // super::_level6::main().await;  // Uncomment when Level 6 is ready
}