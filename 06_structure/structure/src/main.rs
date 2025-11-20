// struct Person {
//     name: String,
//     age: u8,
// }

// fn main() {
//     println!("Hello, world!");
//     let p1: Person = Person {
//         name: String::from("Debottam Kar"),
//         age: 22,
//     };
//     println!("{} {}", p1.name, p1.age);
// }

// struct Person {
//     Name: String,
//     Age: u8,
// }

// impl Person {
//     // A simple method to greet
//     fn greet(&self) {
//         println!("Hello, my name is {}!", self.Name);
//     }

//     // A method that returns age next year
//     fn age_next_year(&self) -> u8 {
//         self.Age + 1
//     }

//     // An associated function (doesn't take &self)
//     fn new(name: &str, age: u8) -> Person {
//         Person {
//             Name: String::from(name),
//             Age: age,
//         }
//     }
// }

// fn main() {
//     let p1 = Person::new("Debottam Kar", 22);

//     p1.greet(); // calls a method
//     println!("Next year I will be {}.", p1.age_next_year());
// }

// struct Person {
//     Name: String,
//     Age: u8,
// }

// impl Person {
//     // Method to increment age (requires &mut self)
//     fn have_birthday(&mut self) {
//         self.Age += 1;
//         println!("Happy Birthday, {}! You are now {}.", self.Name, self.Age);
//     }
// }

// fn main() {
//     let mut p1 = Person {
//         Name: String::from("Debottam Kar"),
//         Age: 22,
//     };

//     // Initially
//     println!("{} is {} years old.", p1.Name, p1.Age);

//     // Change value
//     p1.have_birthday(); // mutably borrows & changes Age

//     // After change
//     println!("{} is now {} years old.", p1.Name, p1.Age);
// }

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function (constructor-like)
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Method using an immutable reference
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // Method using a mutable reference
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    // Method that takes ownership
    fn into_tuple(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

fn main() {
    let mut p = Point::new(3.0, 4.0);

    println!("Distance: {}", p.distance_from_origin());

    p.move_by(1.0, -2.0);
    println!("Moved: {:?}", p.distance_from_origin());

    let t = p.into_tuple();
    println!("Tuple: {:?}", t);
}
