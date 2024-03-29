//Conditionals -- used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    //If/Else
    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What drink would you like to have?");
    }else if age < 21 && check_id{
        println!("Bartender: Sorry, you have to leave.");
    }else {
        println!("Bartender: I'll need to see your ID");
    }

    //Shorthand If
    let is_of_age = if age > 21 { true } else { false };
    println!("Is of age: {}", is_of_age);    
}