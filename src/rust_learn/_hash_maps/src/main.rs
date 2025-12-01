fn main() {
    use std::collections::HashMap; // Import HashMap from standard library
    
    // Create a new HashMap
    let mut scores = HashMap::new(); // New HashMap

    scores.insert(String::from("Blue"), 10); // Insert key-value pair
    scores.insert(String::from("Yellow"), 50); // Insert another key-value pair

    let team_name = String::from("Blue");

    let score = scores.get("Blue"); // Get value for key "Blue"

     // Get value for key team_name ("Blue") and handle None case 
    let score1 = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores { // Iterate over key-value pairs
        println!("{}: {}", key, value);
        println!{"{key}"};
    }
}
