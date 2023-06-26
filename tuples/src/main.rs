fn main() {
    let my_tuple: (u8, String, f64) = (47, "Hello".to_string(), 3.1415926535);

    println!("Name: {}", my_tuple.1);

    // We can also use destructuring
    let(v1, v2, v3) = my_tuple;
    println!("v1: {}, v2: {}, v3: {}", v1, v2, v3);
}
