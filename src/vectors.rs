// Vectors - Resizable arrays

pub fn run() {
    // Vector Definition
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[1] = 20;

    // Add on to vector
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("Vector of numbers: {:?}", numbers);
    println!("Single Value width Debug: {:?}", numbers[0]);
    println!("Single Value: {}", numbers[0]);
    println!("Vector Length: {}", numbers.len());

    // Arrays are stacked allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

    // Loop through vector values
    for i in numbers.iter() {
        println!("Number: {}", i);
    }

    // Loop & mutate values
    for i in numbers.iter_mut() {
        *i *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
