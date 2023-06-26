fn main() {
    let age = 8;
    let can_vote = if age >= 18 {
        true
    } else {
        false
    };
    let can_marry = age >= 18;
    
    if (age >=1) && (age < 18) {
        println!("Child");
    } else if (age >= 18) && (age < 65) {
        println!("Adult");
    } else {
        println!("Elder");
    }
    println!("Can vote: {}", can_vote);
    println!("Can marry: {}", can_marry);
}
