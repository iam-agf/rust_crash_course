enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Does something depending on the input
    match m {
        Movement::Up => println!("Avatar Up"),
        Movement::Left => println!("Avatar Left"),
        Movement::Down => println!("Avatar Down"),
        Movement::Right => println!("Avatar Right"),
    }
}

pub fn run() {
    let avatar1: Movement = Movement::Left;
    let avatar2: Movement = Movement::Right;
    let avatar3: Movement = Movement::Down;
    let avatar4: Movement = Movement::Up;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
