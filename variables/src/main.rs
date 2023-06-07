fn main() {
    const ONE_MIL: u32 = 1_000_000; // We can use "_" to separate constants, which is nice.
    const PI: f32 = 3.1415926535;
    let age  = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age was not a number!");
    // Rust allows to have variables with the same name in different data types.
    // Which is called shadowing
    age += 1;
    println!("I'm {} years old and I want ${}", age, ONE_MIL);
}
