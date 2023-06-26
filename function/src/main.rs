
fn get_sum(x: i32, y: i32) -> i32 {
    x+y
}

fn get_sum2(x: i32, y: i32) -> i32 {
    return x+y;
}

fn get_2(x: i32) -> (i32, i32) {
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for val in list { 
        sum += val;
    }
    sum
}

fn main() {
    let (x1, x2) = get_2(12);
    println!("x1 = {}, x2 = {}", x1, x2);

    let num_list: Vec<i32> = vec![1,2,3,4,5];
    println!("Sum of list {}", sum_list(&num_list));
}
