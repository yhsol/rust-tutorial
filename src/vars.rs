pub fn run() {
    let name = "Brad";
    let mut age = 37;
    age = 38;
    println!("name is {}, age is {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (rust, cargo) = ("rust", "cargo");
    println!("{} with {}", rust, cargo);
}
