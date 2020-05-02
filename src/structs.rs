// Tradition Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }
// Tuple Struct
// struct TupleColor(u8, u8, u8);
// Struct and Function
struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 0,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 255;
    // c.green = 255;
    // c.blue = 255;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut tc = TupleColor(0, 0, 0);

    // tc.0 = 255;
    // tc.1 = 255;
    // tc.2 = 255;

    // println!("TupleColor: {} {} {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("Love", "Jesus");
    // p.first_name = "Jesus".to_string();
    p.last_name = "Jesus".to_string();
    println!("{}", p.full_name());
    p.set_last_name("God");
    println!("{}", p.full_name());

    println!("{:?}", p.to_tuple());
}
