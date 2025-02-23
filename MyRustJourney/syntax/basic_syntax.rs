// This is a basic comment.
// and this is how to define a function
fn compute(a: u32, b: u32) -> u32 {
    // A function must return the same value
    // as the parameters, and its not necesarry
    // to explicit type return
    a + b * 2
}

fn main() {
    let a = 2;
    let b = 3;
    let c = compute(a, b);
    println!("{} + {} * 2 = {}", a, b, c);
}
