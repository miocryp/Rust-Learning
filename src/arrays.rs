use std::mem;

pub fn run() {
    let mut numbers: [i32;5] = [1, 2, 3, 4, 5];
    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);
    // Get single val
    println!("Single Value: {}", numbers[0]);

    // get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}