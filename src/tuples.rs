pub fn run() {
    let person: (&str, &str, i32) = ("Brad", "Mass", 37);

    println!("{}, {}, {}", person.0, person.1, person.2);
}
