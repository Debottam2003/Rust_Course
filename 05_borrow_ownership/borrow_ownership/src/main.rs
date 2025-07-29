// Rust gives the memory safety
// Ownership can be transferred or can be only one mutable reference at a time and any number of immutable reference but not at same time
// Once the mutable reference is out of scope then can create another mutable reference 
fn main() {
    println!("Hello, world!");

    let mut s: String = String::from("Debottam");
    {
     let r: &mut String = &mut s;
     // r.push_str(" world");
     *r = String::from("Okudera"); // overwrite
     // or
     // r.clear();              // remove existing contents
     // r.push_str("okudera");  // append new content
     println!("{} : {}", r, r); // untill r in use cannot use s
    } // <- r goes out of scope here
    println!("{} : {}", s, s); // mutable reference is no more in use so can use s now
    // Owner ship transfered
    let snew = s;
    // Any number of immutable reference
    let r1: &String = &snew;
    let r2: &String = &snew;
    println!("{} : {} : {}", snew, r1, r2);
}
