pub fn main() {
    println!("\n-- 19.3 advanced types\n");
    newtype_for_type_safety_and_abstraction();
}

/// NOTE: think encapsulation here (public vs private API)
fn newtype_for_type_safety_and_abstraction() {
    // Rust also provides a way to create "type aliases"
    // - lose benefit of type-checking using the newtype pattern
    // - primary benefit is to reduce repitition
    type Kilometers = i32; // synonym for i32
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // reducing repitition
    //Box<dyn Fn() + Send + 'static'>
    // BAD
    //let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    //fn take_long_type(f: Box<dyn Fn() + Send + 'static>) {}

    //fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        //// --snip--
    //}

    // GOOD
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("hi"));
    #[allow(dead_code)]
    fn take_long_type(_f: Thunk) {}
    //fn returns_long_type() -> Thunk {}
}
