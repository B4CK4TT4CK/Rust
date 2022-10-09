// Variables holds primitive data or refrences to data
// Variables are immutable by default
// Rust is a block-scoped language
pub fn run(){

    let name = "Frey";

    let mut  age = 21;
    println!("My name is {} and my age is {}" , name, age);

    age = 22;

    println!("My name is {} and my age is {}" , name, age);
    // Define const

    const ID: i64 = 001;
    println!("ID = {}", ID);

    //Assign multiple variables
    let (my_name, my_age, my_game) = ("Frey", 21, "RDR2");
    println!("{} is {} who plays {}", my_name, my_age, my_game);
}
