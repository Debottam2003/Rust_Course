use std::io;

fn main() {
    // Print a greeting message
    println!("Hello, world!");

    // Declare immutable variable `apple` and mutable variable `banana`
    let apple: i32 = 5;
    let mut banana: i32 = 4;

    // Uncommenting the next line will cause a compile-time error because `apple` is immutable
    // apple = 6;

    // Update the value of `banana`
    banana = 5;

    // Print the values of `apple` and `banana`
    println!("{}", apple);
    println!("{}", banana);

    // Prompt the user to guess a number
    println!("Guess the number:");
    let mut guess = String::new();

    // Read the user's input and store it in `guess`
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get input");

    // Print the guessed number
    println!("The guessed number is: {}", guess);

    // Print a pattern of asterisks
    for i in 1..6 {
        for j in 1..i + 1 {
            print!("*");
        }
        print!("\n");
    }

    // Initialize a counter variable
    let mut count: i32 = 0;
    println!("...This is the loop...");

    // Loop until `count` reaches 5
    loop {
        println!("Count: {}", count);
        count += 1;

        if count == 5 {
            println!("...Breaking from the loop...!");
            break;
        }
    }

    // Reinitialize the counter variable with a different type
    let mut count: u16 = 0;
    println!("...This is the while loop...");

    // While loop that runs until `count` is less than 5
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // For loop that iterates from 1 to 5
    println!("...This is the for loop...");
    for i in 1..6 {
        println!("Value: {}", i);
    }

    // Declare floating-point variables
    let mut sgpa: f32 = 8.341;
    let cgpa: f32 = 8.141;

    // Get the memory address of `cgpa`
    let address: &f32 = &cgpa;

    // Print the memory locations of `sgpa` and `cgpa`
    println!("memory location of sgpa is {:p}", &sgpa);
    println!("memory location of cgpa is {:p}", address);

    // Update the value of `sgpa`
    sgpa = 8.345;

    // Declare two integer variables
    let mut a: i8 = 5;
    let mut b: i8 = 10;

    // Print the values of `a` and `b`
    println!("{} {}", a, b);

    // Call the function to swap values by value
    swap_value(a, b);
    println!("Call by value {} {}", a, b);

    // Call the function to swap values by reference
    swap_reference(&mut a, &mut b);
    println!("Call by reference {} {}", a, b);
}

// Function to swap values by value
fn swap_value(mut a: i8, mut b: i8) {
    let temp: i8 = a;
    a = b;
    b = temp;
    println!("{} {}", a, b);
}

// Function to swap values by reference
fn swap_reference(a: &mut i8, b: &mut i8) {
    let temp: i8 = *a;
    *a = *b;
    *b = temp;
    println!("{} {}", *a, *b);
}