pub fn run(){

    use std::mem;

    let mut numbers: [i32;5] = [1,2,3,4,5];

    println!("Numbers :- {:?}", numbers);

    // replace
    numbers[3] = 69;

    // second value
    println!("Second :- {}", numbers[1]);
    println!("Numbers :- {:?}", numbers);

    // arrays len 
    println!("Len: - {}", numbers.len());

    // memory stored 
    println!("Memory :- {}", mem::size_of_val(&numbers));

    // slice 
    let slice: &[i32] = &numbers;
    println!("Slice {:?}", slice);
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice);

}
