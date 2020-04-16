pub fn run() {
    let mut hello = String::from("Hello");

    //Get lenght
    println!("{}", hello.len());

    //Push string
    hello.push_str(" W");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Contains
    println!("Contains 'World' {}", hello.contains("World"));

    //loop through
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    //create sting with capacity
    let mut s = String::with_capacity(10);
    s.push('a');

    // assertion testing
    assert_eq!(1, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
