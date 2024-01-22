//vectors -- resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    //pop off the last value
    numbers.pop();

    println!("{:?}", numbers);

    //get single value
    println!("Single value: {}", numbers[0]);

    //vector lemgth
    println!("Number of elements in Vector: {}", numbers.len());

    // vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[2..4];
    println!("Slice: {:?}", slice);

    //loop through vector values
    for i in numbers.iter() {
        println!("Number: {}", i);
    }

    //loop and mutate values
    for i in numbers.iter_mut() {
        *i *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}   