pub fn run() {
    let mut count = 0;

    // Infinite loop
    // loop {
    //     count += 1;
    //     println!("{}", count);

    //     if count == 20{
    //         break
    //     }
    // }

    println!("Fizzbuzz");

    // While
    count = 0;
    // while count < 100{
    //     if count % 15 == 0{
    //         println!("FizzBuzz");
    //     }
    //     else if count % 5 == 0{
    //         println!("Buzz");
    //     }
    //     else if count % 3 == 0{
    //         println!("Fizz");
    //     }
    //     else{
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }

    // for range loop
    for count in 1..100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", count);
        }
    }
}
