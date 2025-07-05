/*
Print is handled using the following methods
format! - write formatted text to string
print! - same as format but the string is printed to console
println! - same as print! but with a new line
eprint! - same as print! but printed to error console
eprintln! - same as eprint1 but with a new line
*/

use std::fmt;

struct Person{
    name: String,
    age: u32,
}

// Implementing display trait for the struct
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {name}, age: {age}", name=self.name, age=self.age)
    }
}

fn main() {
    println!("{} days", 31);

    // using positional arguments
    println!("{1} means {0}, and {0} means {1}", "love", "coding");

    // using named arguments
    println!("{subj} {pred} {obj}", subj="iphone", pred="isa", obj="phone");

    // using format characters - base 10, 2, 16 respectively
    println!("Base 10 {0}",  16);
    println!("Base 2 {:b}",  16); 
    println!("Base 16 {:x}", 16);

    // justification
    // right justification
    println!("Right Justification: {number:0>5}", number=1);

    // left justification
    println!("Left Justification: {number:0<5}", number=1);

    // sending justification as a param
    println!("Left Justification: {number:0<width$}", number=1, width=10);

    // only types implement fmt::Display can be formatted with {}
    // User-defined types doen't implment fmt::Display by default
    // check how we define Person structure above
    // its done by implementing the std::fmt trait.

    // Note: implementing the fmt::Display trait automatically implements the ToString trait
    // which allows us to convert any type to String

    let person =  Person {
        name: String::from("Alice"),
        age: 32,
    };

    println!("Person Info: {}", person);
}