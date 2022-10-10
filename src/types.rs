
    /*
     * integers i32
     * floats = f32, f64
     * bool
     * char
     * tuples
     * arrays
     */
pub fn run(){

    // int = i32
    let x = 1;

    // flaot = f64
    let y = 2.14;

    //explict type 

        let z: i64 = 4579373928793;

    println!("Max i32 : {}", std::i32::MAX);
    println!("Max i64 : {}", std::i64::MAX);

    // bool
    let is_active = true;
    println!("{:?}", (x,y,z, is_active));

    let is_greater = 10 > 20;
    println!("{:?}", (is_greater));

    let a1 = 'F';
    let face = '\u{1f600}';
    println!("{}", a1);
    println!("{}", face);

}
