fn main() {
    let arr_1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("1st: {}", arr_1[0]);
    println!("lenght: {}", arr_1.len());

    let mut loop_idx: usize = 0;
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 5 {
            break;
        }
        println!("Value: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    loop_idx = 0;
    while loop_idx < arr_1.len() {
        println!("Value: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    println!("For loop: ");
    for val in arr_1.iter() {
        println!("Value: {}", val);
    }

}
