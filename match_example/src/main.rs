use std::cmp::Ordering;

fn main() {
    let age = 18;
    match age {
        0..=18 => println!("Child"),
        19..=65 => println!("Adult"),
        _ => println!("Elder"), // similar to defalt case
    }
    let voting_age = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Too young"),
        Ordering::Greater => println!("Too old"),
        Ordering::Equal => println!("You are old enough"),
    }
}
