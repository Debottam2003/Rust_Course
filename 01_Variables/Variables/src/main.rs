fn main() {
    let v1: i8 = 8;
    let v2: i16 = 16;
    let v3: i32 = 32;
    let v4: i64 = 64;
    let v5: i128 = 128;
    let v6: u8 = 8;
    let v7: u16 = 16;
    let v8: u32 = 32;
    let v9: u64 = 64;
    let v10: u128 = 128;
    let v11: f32 = 32.0;
    let v12: f64 = 64.0;
    let v13: char = 'a';        
    let v14: bool = true;
    let v15: &str = "Hello, world!";
    let v16: String = String::from("Hello, world!");
    let v17: [i32; 5] = [1, 2, 3, 4, 5];
    let v18: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v21: (i32, f64, char) = (42, 3.14, 'a');
    
    // Print signed integers
    println!("v1 (i8): {}", v1);
    println!("v2 (i16): {}", v2);
    println!("v3 (i32): {}", v3);
    println!("v4 (i64): {}", v4);
    println!("v5 (i128): {}", v5);

    // Print unsigned integers
    println!("v6 (u8): {}", v6);
    println!("v7 (u16): {}", v7);
    println!("v8 (u32): {}", v8);
    println!("v9 (u64): {}", v9);
    println!("v10 (u128): {}", v10);

    // Print floating point numbers
    println!("v11 (f32): {}", v11);
    println!("v12 (f64): {}", v12);

    // Print character
    println!("v13 (char): {}", v13);

    // Print boolean
    println!("v14 (bool): {}", v14);

    // Print string slices and String
    println!("v15 (&str): {}", v15);
    println!("v16 (String): {}", v16);

    // Print array
    println!("v17 ([i32; 5]): {:?}", v17);

    // Print vector
    println!("v18 (Vec<i32>): {:?}", v18);

    // Print tuple
    println!("v21 ((i32, f64, char)): {:?}", v21);
}
