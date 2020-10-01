/// 18.1 - conditional `if let` expressions
/// https://doc.rust-lang.org/stable/book/ch18-01-all-the-places-for-patterns.html
fn main() {
    if_let_expressions();
    while_let_conditional_loops();
    patterns_in_for_loops();
    let_pattern_match();
    function_pattern_matching();
}

fn if_let_expressions() {
    println!("\n----- 18.1 patterns and matching : `if let` expressions\n");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse(); // parses a string into a unsized 8-bit integer
    // NOTE: big downside of `if let` expressions is that the compiler doesn't check exhaustiveness
    // compiler can't alert us of a possible logic bug, i.e., missing the last `else` block
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { // introduces new shadowed `age` variable
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let_conditional_loops() {
    // `while let` conditional loops
    // - following example uses a vector as a stack
    println!("\n----- 18.1 patterns and matching : `while let` conditional loops\n");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // loop runs until the stack is empty
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn patterns_in_for_loops() {
    println!("\n----- 18.1 patterns and matching : using a pattern in a for-loop\n");
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("char {:?} is at index {}", value, index);
    }
}

fn let_pattern_match() {
    println!("\n----- 18.1 patterns and matching : lets are similar to Elixir's match operator `=`\n");
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn function_pattern_matching() {
    println!("\n----- 18.1 patterns and matching : function pattern matching (similar to functional langs)\n");

    // declares a function name `foo` that takes one parameter name `x` of type `i32`
    #[allow(dead_code, unused_variables)]
    fn foo(x: i32) {}

    // similar to Elixir and other functional langs
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}
