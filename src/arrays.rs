use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);
    println!("Single Value: {}", numbers[0]);
    println!("Array Length: {}", numbers.len());
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..2];

    println!("Slice {:?}", slice);
}
