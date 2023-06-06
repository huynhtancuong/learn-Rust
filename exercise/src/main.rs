fn main() {
    print_fizz_buzz_to(100);
}

fn print_fizz_buzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizz_buzz(i));
    }
}

fn fizz_buzz(n: u32) -> String {
    let fizz:&str = if is_divisible_by(n, 3) {"Fizz"} else {""};
    let buzz:&str = if is_divisible_by(n, 5) {"Buzz"} else {""};
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}")
    }
    format!("{fizz}{buzz}")
}
/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
fn is_divisible_by(n: u32, divisor: u32) -> bool {
    n % divisor == 0
}