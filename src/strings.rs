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
    println!("{}",hello);
    
}

