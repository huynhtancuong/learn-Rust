fn main() {
    // We have 2 types of string
    // 1. String -> can be changed. It's a vector of bytes
    // 2. &str -> can't be changed. It's a reference to a string in memory.

    let mut st1 = String::new();
    st1.push('A'); // push a character
    st1.push_str(" word"); // push a string
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    // We can create a string from string
    let st3 = String::from("h e l l o w o r l d");
    // We can create a vector from a string 
    let mut v1: Vec<char> = st3.chars().collect(); 
    v1.sort(); // sort alphabetically
    v1.dedup(); // remove duplicates
    for char in v1 {
        println!("{}", char);
    }

    // 2. &str
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    // convert string to bytes array
    let _byte_arr1 = st5.as_bytes();
    let st6: &str = &st5[..6];
    println!("{}", st6.len());
    st5.clear();

    // Concatenate strings
    let st7 = String::from("Hello ");
    let st8 = String::from("world!");
    let st7 = st7 + &st8; // we input the reference to the string st8.
    let st9 = st7 + &st8; // st7 is borrowed. st8 is not borrowed.
    println!("{}", st9);

    for char in st9.bytes() {
        println!("{}", char);
    }


}
