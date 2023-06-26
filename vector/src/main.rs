fn main() {
    // Vector is a array which can grow if muttable.
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4,5];
    vec2.push(5);
    println!("1st: {}", vec2[0]);

    // We have 2 ways to get the second element
    match vec2.get(5) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value")
    }
    // or
    let another_second: &i32 = &vec2[1];

    // If we want to cycle through the vector and multiply every element by 2
    for i in &mut vec2 {
        *i = *i*2; // we need to dereference first
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vector length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());

}
