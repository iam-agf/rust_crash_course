pub fn run(){

    // Immutable strings are defined like:
    let hello1 = "Hello";

    // Growable can be
    let mut hello = String::from(hello1);
 
    println!("{}", hello1.len());

    // Push only accept chars
    hello.push('0');
    println!("{}", hello);
    
    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // push_str accepts strings
    hello.push_str("World");
    println!("{}", hello);    
 
    // is_empty bool function to check if variable is ''
    let empty: bool = hello.is_empty();
    println!("{}", empty);

    // contains to check if string a is in string b
    let container: bool = hello.contains("world");
    println!("{}", container);    
    
    let mut text2 = String::from("huahuahuahuahua");
    text2 = text2.replace("hua", "evm");

    println!("{}", text2);

    for word in "a b c d e".split_whitespace(){
        println!("{}", word)
    }

    let mut s = String::with_capacity(10);
    s.push('1');
    s.push('2');
    
    assert_eq!(2, s.len());
    assert_eq!(3, s.len());

    println!("{}", s)


}