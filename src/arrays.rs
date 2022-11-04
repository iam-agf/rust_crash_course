use std::mem;

pub fn run(){
    let numbers:[i32;5] = [1,2,3,4,5];

    // You can print the whole array
    println!("{:?}", numbers);

    // Get one element
    println!("{}", numbers[0]);
    
    // Get size of array
    println!("{}", numbers.len());
    
    // Arrays are size allocated
    println!("Array occupies this size: {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];

    println!("{:?}", slice)
}