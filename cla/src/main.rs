// use std::env;
// fn main() {
//     println!("Hello, world!");
//     let arr: Vec<String> = env::args().collect();
//     for i in 0..arr.len() {
//         print!(" {} ", arr[i]);
//     }
//     let arr: [i32; 5] = [7, 8, 9, 2, 3];
//     println(arr.len());
// }
fn main() {
    println!("Hello, world!");
    let day: &str = "march"; // Change this to test different months

    match day {
        "january" => println!("This is January."),
        "february" => println!("This is February."),
        "march" => println!("This is March."),
        "april" => println!("This is April."),
        "may" => println!("This is May."),
        "june" => println!("This is June."),
        "july" => println!("This is July."),
        "august" => println!("This is August."),
        "september" => println!("This is September."),
        "october" => println!("This is October."),
        "november" => println!("This is November."),
        "december" => println!("This is December."),
        _ => println!("Invalid month."),
    }
}
