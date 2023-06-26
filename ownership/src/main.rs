
fn swap_str(s1: String, s2: String) -> (String, String) {
    return (s2, s1);
}

fn main() {
    let mut s1: String = String::from("Hello");
    let mut s2: String = String::from("World");

    (s1, s2) = swap_str(s1, s2);

    println!("s1: {:?}", s1);
    println!("s2: {:?}", s2);
}
