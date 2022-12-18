pub fn run() {
    let name = "Brandon";
    let mut age = 35;
    println!("My name is {} and I am {}", name, age);
    age = 36;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("Brandon", 40);
    println!("{} is {}", my_name, my_age);
}