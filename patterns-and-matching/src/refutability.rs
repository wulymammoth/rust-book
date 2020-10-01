pub fn main() {
    println!("\n=== 18.2 patterns and matching | refutability ===\n");
    refutable_pattern();
    println!("\n=== 18.3 patterns and matching | pattern syntax ===\n");
    pattern_syntax();
}

fn refutable_pattern() {
    let some_option_value: Option<u8> = Some(5);

    // pattern `None` is not covered, so instead of...
    // let Some(x) = some_option_value;

    // use `if let`
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}

fn pattern_syntax() {
    println!("\n-- matching literals\n");
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("not one, two, or three"),
    }

    println!("\n-- matching named variables\n");
    // - NOTE: `match` starts a new scope
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    println!("\n-- multiple patterns\n");
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("\n-- matching ranges of values with `..=`\n");
    // allows us to match an inclusive range of values
    let x = 5;
    match x {
        1..=5 => println!("x's value is: {}, output: one through five", x),
        _ => println!("something else"),
    }

    println!("\n-- matching ranges of `char` values with `..=`\n");
    let x = 'c';
    match x {
        'a'..='j' => println!("x's value is: {}, an early ASCII letter", x),
        'k'..='z' => println!("x's value is: {}, a late ASCII letter", x),
        _ => println!("somethi&ng else"),
    }

    println!("\n-- destructuring to break apart values\n");
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // destructuring (explicit)
    assert_eq!(0, a);
    assert_eq!(7, b);

    println!("\n-- destructuring w/ more convenience (like JS), but listing only the names of the fields we care about\n");
    let Point { x, y } = p; // destructuring (shorthand)
    assert_eq!(0, x);
    assert_eq!(7, y);

    println!("\n-- destructuring w/ pattern matching\n");
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    println!("\n-- destructuring enums\n");
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    println!("\n-- destructuring nested structs and enums\n");
    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[allow(dead_code)]
    enum Msg {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Msg::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Msg::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Msg::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    println!("\n-- destructuring structs and tuples\n");
    let ((_feet, _inches), Point { x: _x, y: _y }) = ((3, 10), Point { x: 3, y: 10 });

    println!("\n-- ignoring values in a pattern\n");
    fn foo(_: i32, y: i32) {
        println!("This code only usese the y parameter: {}", y);
    }
    foo(3, 4);

    println!("\n-- ignoring parts of a value with a nested _\n");
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customize value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    println!("\n-- ignoring multiple parts with underscores in multiple places\n");
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    println!("\n-- subtle difference in the use of underscores\n");
    // underscore prefixed to a var name still binds
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }

    // underscore with no suffix does NOT bind
    println!("{:?}", s);

    println!("\n-- ignoring remaingin parts of a value with ..\n");
    #[allow(dead_code)]
    struct Pnt {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Pnt { x: 0, y: 0, z: 0 };
    match origin {
        Pnt { x, .. } => println!("x is {}", x),
    }

    println!("\n-- .. with a tuple\n");
    let numbers = (2, 4, 8, 15, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    println!("\n-- extra conditionals with match guards (just like Elixir)\n");
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than fives: {}", x), // notice the guard
        Some(x) => println!("{}", x),
        None => (),
    }

    println!("\n-- match guards can solve our pattern-shadowing problem\n");
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Match, n = {}", n), // no need to shadow here
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    println!("\n-- we can also use the pipe (|) in a match guard to specify multiple patterns\n");
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    println!("\n-- @ bindings\n");
    // the *at* operator lets us create a variable that holds a value at the same time we're
    // testing that value to see whether it matches a pattern
    enum Mezzage {
        Hello { id: i32 },
    }
    let msg = Mezzage::Hello { id: 5 };
    match msg {
        Mezzage::Hello {
            id: id_variable @ 3..=7, // like Elixir's operator
        } => println!("Found an id in range: {}", id_variable),
        Mezzage::Hello { id: 10..=12 } => {
            println!("Found an id in aonother range");
        }
        Mezzage::Hello { id } => println!("Found some other id: {}", id),
    }
}
