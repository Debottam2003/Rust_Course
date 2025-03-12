use std::io;
fn main() {
    println!("Hello, world!");
    let apple: i32 = 5;
    let mut banana: i32 = 4;
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
        print!("\n");
        // println!("");
    }
    let mut count: i32 = 0;
    println!("...This is the loop...");
    loop {
        println!("Count: {}", count);
        count += 1;

        if count == 5 {
            println!("...Breaking from the loop...!");
            break;
        }
    }
    let mut count: u16 = 0;
    println!("...This is the while loop...");
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
    println!("...This is the for loop...");
    for i in 1..6 {
        println!("Value: {}", i);
    }
    let mut sgpa: f32 = 8.341;
    let cgpa: f32 = 8.141;
    println!("memory location of sgpa is {:p}", &sgpa);
    println!("memory location of cgpa is {:p}", &cgpa);

}
