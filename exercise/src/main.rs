fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: bool = true;

    println!("{x} * {y} = {}", multiply(i16::from(x), y.into()));
}