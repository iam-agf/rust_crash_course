pub fn run() {
    hey("Hey", "Joe");

    // Bind function variables to functions
    let get_add = add(3,4);
    println!("{}", get_add);

    let n3: i32 = 20;
    let add_nums = |n1:i32,n2:i32| n1+n2+n3;
    println!("{}", add_nums(5,5))
}


fn hey(greet: &str, name: &str){
    println!("{} {}, nice to meet you",greet, name)
}

fn add(num1: i32, num2: i32) -> i32{
    num1+num2
}