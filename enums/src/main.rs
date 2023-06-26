fn main() {
    enum Day {
        Mon, Tue, Wed, Thu, Fri, Sat, Sun
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Sat | Day::Sun => true,
                _ => false
            }
        }
    }

    let today: Day = Day::Sun;
    match today {
        Day::Sun => println!("Today is sunday!!!"),
        _ => println!("Today we have to work")
    }
    if today.is_weekend() {
        println!("We don't have to work today!!");
    }
}
