// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut p = Person::new("John", "Doe");


    // println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());

    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person {:?}", p.to_tuple());

    // let mut color: Color = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // color.blue = 200;
    // println!("{:?}", (color.red, color.blue, color.green));

    // let mut color: Color = Color(255, 0, 0);

    // color.2 = 200;
    // println!("{:?}", (color.0, color.1, color.2));
}
