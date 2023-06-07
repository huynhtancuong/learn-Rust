fn main() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.11111111111111111111);
    // f32 has the precision is 6 digits
    // f64 has the precision is 15 digits
}
