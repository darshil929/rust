//variables hold primitive data or reference to data
//variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
    let name  = "Darshil"; 
    let mut age = 21;
    println!("My name is {} and I am {}", name, age);
    age = 22;
    println!("My name is {} and I am {}", name, age);

    //define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple variables
    let (my_name, my_age) = ("Darshil", 21);

    println!("{} is {}", my_name, my_age)
}