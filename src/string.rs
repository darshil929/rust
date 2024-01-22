//Primitive strings = Immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let _hello = "Hello";

    let mut hey = String::from("Hey ");

    //Get length
    println!("Length: {}", hey.len());

    //push char
    hey.push('w');

    //push strig
    hey.push_str("orld");

    //capacity in bytes
    println!("Capacity: {}", hey.capacity());

    //is_empty
    println!("Is Empty? {}", hey.is_empty());

    //contains
    println!("Contains word 'World'? {}", hey.contains("World"));

    //replace
    println!("Replace: {}", hey.replace("world", "there"));

    //loop through string by white space
    for word in hey.split_whitespace() {
        println!("{}", word);
    }

    println!("{}", hey);
}