pub fn run() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    println!("{}", 10);
    // Basic formatting
    println!("{} is my friend, from {}", "Brandon", "USA");
    // Positional Argument
    println!("{0} is from {1} and {0} likes to {2}", "Brandon", "USA", "Code");
    // Named Argument
    println!("{name} likes to play {activity}", name = "Bradon", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:0}", 10, 10, 10);
}