pub fn run() {
    //Print to console
    println!("hello, from print.rs file");

    //Basic Formatting
    println!("Number: {} and {}", 1, "two");

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Ducky", "New York", "code"
    );

    //Named Arguments
    println!(
        "{name} likes to {activity}",
        name = "Ducky",
        activity = "swim"
    );

    //Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
    //Basic Math
    println!("10 + 10 = {}", 10 + 10)
}
