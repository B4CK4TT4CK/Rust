pub fn run(){

    // print 
    println!("Hello there twitter = print.rs");
    println!("Number :-{}", 42 );

    // formatting 
    println!("{} is from {}", "Frey", "New Delhi");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}",
             "Pinchu", "India", "Code");

    // Name Arguments
    println!("{name} likes to play {game}",
             name = "Frey", game = "RDR2");

    // Placeholder 
    println!("Binary: {:b} Hex {:x} Octal {:o}", 
             10, 6018, 8);
    // Placeholder Debug trail
    println!("{:?}", (12, true, "frey"));

    // Baic Math
    println!("{} + {}", 12+12, 42-32);
    println!("the sum of 2 + 2 is {}", 2+2);

}

