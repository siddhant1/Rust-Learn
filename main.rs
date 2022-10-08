// fn main() {
//     println!("Testing");
// }

fn main() {
    let subject = "World";

    let a = 1.1;
    let mut b = 2.2;

    b = 4.4;
    // b is immutable, you can't mutate any variable defined with b

    println!("{},{}!  {}", "Hello", subject, a * b);

    // panic! throws error

    println!("{}!", subject)
}
