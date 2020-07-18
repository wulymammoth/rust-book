fn main() {
    another_function(5, 6);
    println!("The value of calling the `five` function is {}", five());
    let x= 7;
    println!("The value of invoking `plus_one` on {}, is {}", x, plus_one(7));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(num: i32) -> i32 {
    num + 1
}
