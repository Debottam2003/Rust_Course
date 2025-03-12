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
    let address: &f32 = &cgpa;
    println!("memory location of sgpa is {:p}", &sgpa);
    println!("memory location of cgpa is {:p}", address);
    sgpa = 8.345;
    let mut a: i8 = 5;
    let mut b: i8 = 10;
    println!("{} {}", a, b);
    swap_value(a, b);
    println!("Call by value {} {}",a, b);
    swap_reference(&mut a, &mut b);
    println!("Call by reference {} {}",a, b);
}
fn swap_value(mut a: i8, mut b: i8){
   let temp: i8 = a;
   a = b;
   b = temp;
   println!("{} {}",a, b);
}
fn swap_reference(a: &mut i8, b: &mut i8){
    let temp: i8 = *a;
    *a = *b;
    *b = temp;
    println!("{} {}",*a, *b);
 }