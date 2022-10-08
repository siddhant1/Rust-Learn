use std::io;

fn main() {
    println!("Enter your number");
    let mut guess = String::new();

    io::stdin.read_line(&guess).expect((msg));

    println!({}, guess)
}
