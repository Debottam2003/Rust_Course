mod secondary; // Declare the module ( tell the compiler to look for secondary.rs in the same folder )
mod inner { // nested mod import
    pub mod nested;
}
use secondary::sub; // Bring the public function into scope
use inner::nested::mul;

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    println!("Hello, world!");
    println!("{}", add(10, 5));
    println!("{}", sub(10, -5));
    println!("{}", mul(2 , 3));
}
