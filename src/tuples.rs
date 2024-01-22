//Tuples group together values of different types
//Max 12 elements


pub fn run() {
    let person: (&str, &str, i8) = ("Darshil", "Mumbai", 21);

    println!("{} from {} is {}", person.0, person.1, person.2);
}