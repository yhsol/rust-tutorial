pub fn run() {
    // Print to console
    println!("Hello World! from the print.rs file");

    println!("Number: {}", 1);

    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    println!(
        "{name} likes to play {activity}",
        name = "Billy Bean",
        activity = "Baseball"
    );

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // using Debug trait (std::fmd::Debug)
    println!("{:?}", (12, true, "hello"));

    println!("5 + 5 = {}", 5 + 5);
}
