pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let knows_the_guy: bool = true;

    if age >= 21 && check_id || knows_the_guy{
        println!("Bartender: What's up");
    }
    else if age < 21 && check_id{
        println!("Bartender: Sorry, you have to leave");
    }
    else{
        println!("Bartender: Show me your id");
    }
}