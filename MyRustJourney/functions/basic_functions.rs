// Pure function, this  type of functions doesnt
// depend of external states. And doesnt have side efects.

fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let x = 13;
    square(x);
}
