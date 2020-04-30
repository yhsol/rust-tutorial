pub fn run() {
    // Array Definition
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[1] = 20;

    println!("Array of numbers: {:?}", numbers);
    println!("Single Value width Debug: {:?}", numbers[0]);
    println!("Single Value: {}", numbers[0]);
    println!("Array Length: {}", numbers.len());

    // Arrays are stacked allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);
}
