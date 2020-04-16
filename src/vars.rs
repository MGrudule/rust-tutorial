pub fn run() {
    let name = "Ducky";
    let mut age = 12;

    println!("My name is {} and I am {}", name, age);

    age = 38;
    println!("My name is {} and I am {}", name, age);

    //Define costant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Ducky", 22);
    println!("My name is {} and I am {}", my_name, my_age);
}
