use std::io;
fn main() {
    println!("Hello, world!");
    let apple = 5;
    let mut banana = 4;
    //apple = 6;
    banana = 5;
    println!("{}", apple);
    println!("{}", banana);
    println!("Guess the number:");
    let mut guess = String::new();
    io::stdin()
              .read_line(&mut guess)
              .expect("Falied to get input");
    println!("The guessed number is: {}", guess);
    for i in 1..6{
        for j in 1..i + 1{
            print!("*");
        }
        println!("");
    }
}
