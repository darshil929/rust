pub fn run() {
    //print to console
    println!("Hello from the print.rs file");

    //Basic fromatting
    println!("Number: {}", 1);

    println!("{} lives in {}", "Darshil", "Ghatkopar");

    //Positional Arguments
    println!("{0} lives in {1} and {0} likes to {2}", "Darshil", "Ghatkopar", "code");

    // Named Arguments
    println!("{name} likes to code {activity}", name = "Darshil", activity = "Rust");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}