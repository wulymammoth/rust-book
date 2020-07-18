fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    counter_example();
    for_loop_ex();
    for_loop_reverse_range_ex();
}

fn counter_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn for_loop_ex() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_loop_reverse_range_ex() {
    for num in (1..5).rev() {
        println!("countdown: {}", num);
    }
}
