pub fn run() {
    // Print to console
    println!("Hello from the print file!");

    // Basic Formatting
    println!("{} is from {}", "Brad", "Mass");

    // Positional parameters
    println!("{0} is from {1} and {2} likes to {1}", "Brad", "Mass", "Code",);

    // Named arguments
    println!("{name} lines to play {activity}",
    name = "Brad",
    activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b}\nHex: {:x}\nOcto: {:o}",10,10,10);

    // Basic Math 
    println!("10 + 10 = {}", 10+10)
}