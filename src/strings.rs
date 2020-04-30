pub fn run() {
    let mut hello = String::from("Hello ");

    // '' = char
    // "" = &str
    hello.push('W');
    hello.push_str("orld!");

    println!("{}", hello.len());

    println!("{}", hello.capacity());

    println!("{}", hello.is_empty());

    println!("{}", hello.contains("World"));

    println!("{}", hello.replace("World!", "world!"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    assert_eq!(2, s.len());

    println!("{}", hello);
}
