pub fn run(){
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and I'm {} years old", name, age);

    age = 38;

    println!("My name is {} and I'm {} years old", name, age);

    const  ID: i32 = 001;

    println!("ID: {}", ID);

    let (my_name, my_age) = ("Chad", 29);

    println!("{} is {}", my_name, my_age)
}