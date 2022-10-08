// // fn main() {
// //     println!("Testing");
// // }

// //  Rust is typechecked
// fn main() {
//     let subject = "World";

//     let a = 1.1;
//     let mut b = 2.2;

//     b = 4.4;

//     // b is immutable, you can't mutate any variable defined with b ( use mut )

//     // (!) this is a macro ka syntax

//     println!("{},{}!  {}", "Hello", subject, a * b);

//     // panic! throws error

//     println!("{}!", subject)
// }

use std::io;

fn main() {
    println!("Enter your number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("msg");

    println!("Your guessed number is {}", guess)
}
