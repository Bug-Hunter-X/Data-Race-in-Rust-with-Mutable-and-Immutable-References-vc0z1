fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        *y = 10;
    }
    let z = x; // Cloning x to avoid simultaneous mutable and immutable references
    println!("x = {}", x);
    println!("z = {}", z);
}