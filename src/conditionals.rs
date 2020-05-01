pub fn run() {
    let age: u8 = 21;
    let check_id: bool = true;
    let identified_person: bool = true;

    if age >= 20 && (check_id || identified_person) {
        println!("welcome");
    } else if age >= 20 && !check_id {
        println!("I'll need your ID",)
    } else {
        println!("leave",)
    }

    let is_of_age = if age >= 20 { true } else { false };
    println!("is of age: {}", is_of_age);
}
