pub fn run() {
    let arr:[i32;6]=[2,5,7,12,11,25];
    let slice = &arr[2..5];
    println!("Slice Array: {:?}", slice);
}