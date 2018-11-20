fn fib(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        m => fib(m - 1) + fib(m - 2),
    }
}
fn main() {
    println!("Fib(5) = {}", fib(5));
}
