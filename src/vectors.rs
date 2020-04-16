use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    //Re-assign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last value
    numbers.pop();

    println!("{:?}", numbers);
    println!("Vector Single Value: {}", numbers[0]);
    println!("Vector Length: {}", numbers.len());
    println!("Vector Occupies {} bytes", mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..2];

    println!("Vector Slice {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    // Loop through vector values & mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Number: {:?}", numbers);
}
