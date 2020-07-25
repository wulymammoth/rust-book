#[derive(Debug)]
enum Message {
    //Quit,
    //Move { x: i32, y: i32 },
    Write(String),
    //ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", &self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("the value for a {:?} is {}", Coin::Penny, value_in_cents(Coin::Penny));
    println!("the value for a {:?} is {}", Coin::Nickel, value_in_cents(Coin::Nickel));
    println!("the value for a {:?} is {}", Coin::Dime, value_in_cents(Coin::Dime));
    println!("the value for a {:?} is {}", Coin::Quarter(UsState::California), value_in_cents(Coin::Quarter(UsState::California)));

    // matching with Option<T>
    let five = Some(5);
    let six = Some(6);
    let none = plus_one(None);

    println!("five {:?}", five);
    println!("six {:?}", six);
    println!("none {:?}", none);

    let some_u8_value = 10u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // unit value (nothing happens)
    }

    // if let/else
    let mut count = 0;
    let coin = Coin::Quarter(UsState::California);
    match coin {
        Coin::Quarter(state) => println!("state quarter from {:#?}", state),
        _ => count += 1,
    }
    println!("the count is {}", count);
}
