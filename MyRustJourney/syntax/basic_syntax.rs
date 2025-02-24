use std::i32;

// When writting a funtcion we specify parameters much like we would in C++
// we have type  signatures and variable names within the pareentheses.
fn greetings(a: &str, b: &str) -> () {
    println!("{}  {}", a, b)
}

// Statements vs. Expressions
// Statements = is a  declaration that doesnt return a value but does something
// Expressions = is whatever that returns a value

fn branch(x: i32) -> i32 {
    if x > 40 {
        // Expression 'if' returns a value
        println!("greater");
        x * 2 // Expression return 'x *  2'
    } else {
        x * 3 // Expression return 'X * 3'
    }
}

fn main() {
    let x = 45; // Statement
    let y = branch(x); // Statement calls the branch function
    greetings("hello", "world")
}
