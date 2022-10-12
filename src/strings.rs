pub fn run(){


    // there r  2 type of strings 
    // Primititive str
    // global str
    

    let mut hello = String::from("Hello Hacker ");
    println!("{}",hello);


    // get len

    println!("Length :- {}",hello.len());

    hello.push('N');
    hello.push_str("xST");
    println!("{}",hello);
// capacity 

    println!("Capacity:- {}", hello.capacity());
// Is EMpty
    println!("Is Empty :- {}", hello.is_empty());
// Contains 
    println!("Contains 'NxsT' ? {}", hello.contains("Nxst"));
    println!("Contains 'NxST' ? {}", hello.contains("NxSt"));
    // replace 
    println!("Replace: {}", hello.replace("NxST", "Frey"));
    //loop
    for word in hello.split_whitespace() {
        println!("{}",word);
    }
    // create string with cpacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // assertion test
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}",s);
    println!("{}",hello);
    
}

