struct Retangle {
    width: u32,
    height: u32,
}

impl Retangle {
    fn new(width: u32, height: u32) -> Retangle {
        Retangle {width, height,}
    }
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn increase_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect:Retangle = Retangle::new(10, 20);
    println!("The area of the rectangle is {}", rect.area());
    rect.increase_width(10);
    println!("The area of the rectangle is {}", rect.area());
}