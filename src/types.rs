pub fn run(){
    // Default i32
    let x = 1;

    // Default f64
    let y = 2.5;

    let z: i64 = 6666666666666666;

    println!("{}",z);

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active: bool = true;

    // We can use boolean statements 
    let is_greater: bool = 10 < 1000;

    let a = 'a';

    let emoji = '\u{1F600}';

    // Strings can be considered also chars and emojis

    println!("{:?}", (x,y,z, is_active, is_greater, a, emoji))
}