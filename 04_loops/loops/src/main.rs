fn main() {
    println!("Hello, world!");

    let mut count = 0;
    loop {
        println!("The current count is: {}", count);
        count += 1;
        if count == 5 {
            break
        }
    }

    count = 0;
    while count < 5 {
        println!("The current count is: {}", count);
        count += 1;
    }

    for i in 0..10 { // upper limit is excluded
        println!("{}", i);
    }
}
