//Arrays - fixed list where elements are same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[2] = 20;

    println!("{:?}", numbers);

    //get single value
    println!("Single value: {}", numbers[0]);

    //array lemgth
    println!("Number of elements in array: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[2..4];
    println!("Slice: {:?}", slice);
}