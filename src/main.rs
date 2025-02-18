// use std::io;
// fn main() {
//     // println!("Hello, world!");
//     println!("Lets guess!");
//     println!("input your guess");
//     let mut guess = String::new();

//     io::stdin()
//     .read_line(&mut guess).expect("Faile to read line!")
//     .expect("failed to get num");

//     println!("your guess is {guess}")
// }

use std::default;


fn main() {
    let logical : bool = true;
    
    let a_float : f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 8;

    let mut inferred_type = 13;
    inferred_type = 32;

    print!("{logical}");
    println!("{a_float}");
    println!("{}",an_integer);
    println!("{}", inferred_type);
    // println!("{}",inferred_type);
    println!("{default_float}");
    println!("{default_integer}");   

    march();
}

fn march () {
    println!("make me proud :>")
}


