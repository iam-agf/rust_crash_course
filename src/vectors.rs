use std::mem;

pub fn run(){
    let mut numbers:Vec<i32> = vec![1,2,3,4,5];

    numbers.pop();
    numbers.push(20);

    // You can print the whole vector
    println!("{:?}", numbers);

    // Get one element
    println!("{}", numbers[0]);
    
    // Get size of vector
    println!("{}", numbers.len());
    
    // vectors are size allocated
    println!("Vector occupies this size: {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];

    println!("{:?}", slice);

    for x in numbers.iter(){
        println!("Number {}", x)
    }
    
    for x in numbers.iter_mut(){
        *x *= 3;
        println!("Number {}", x);
    }

    println!("{:?}", numbers)
}