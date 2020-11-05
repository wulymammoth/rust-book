use oop_gui::{Button, Draw, Screen};

// another dev may want to exhibit `Draw` behavior on their custom type, `SelectBox`
#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // not implement (for demo only)
    }
}

fn main() {
    // NOTE: consumer of the library can now create an instance of  `Screen` and add `Button` and
    // `SelectBox` into its components by putting each of them into a `Box`, making them **trait
    // objects** and are then able to use its interface, `draw()`
    let _screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };
}
