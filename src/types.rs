pub fn run() {
    let _x = 1;
    let _y = 2.5;

    //add Explicit type

    let z: i64 = 3452375893479563479;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;

    //Get boolean from expression
    let is_greater = 10 > 5;

    //char
    let a1 = 'a';
    let face = '\u{1F317}';
    println!("{:?}", (z, is_active, is_greater, a1, face));
}
