pub fn run() {
    let mut count = 0;

    // Infinite Loop
    // loop {
    //     count += 1;
    //     println!("Count: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While Loop (FizzBuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz",)
    //     } else if count % 3 == 0 {
    //         println!("fizz",)
    //     } else if count % 5 == 0 {
    //         println!("buzz",)
    //     } else {
    //         println!("{}", count)
    //     }

    //     // Inc
    //     count += 1;
    // }

    // For Range
    // for i in 0..100 {
    //     if i % 15 == 0 {
    //         println!("fizzbuzz",)
    //     } else if i % 3 == 0 {
    //         println!("fizz",)
    //     } else if i % 5 == 0 {
    //         println!("buzz",)
    //     } else {
    //         println!("{}", i)
    //     }
    // }

    for i in 0..100 {
        if i % 15 == 0 {
            println!("fizzbuzz",)
        } else if i % 3 == 0 {
            println!("fizz",)
        } else if i % 5 == 0 {
            println!("buzz",)
        } else {
            println!("{}", i)
        }
    }
}
