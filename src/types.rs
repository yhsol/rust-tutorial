/*
Primative Types--
Integers: u8, i8, u16, i16, u32, i32, u62, u32, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean: (bool)
Characters: (char)
Tuples
Array
*/

pub fn run() {
    let x = 1;
    let y = 2.5;
    let z: i64 = 545454;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active: bool = true;
    let is_greater: bool = y < x as f32;

    let a1 = 'a';
    let a2 = '\u{270c}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, a2));
}
