use oop::{AveragedCollection, Draw, Screen};

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("TODO: code to draw button");
    }
}

// NOTE: could have been an extension implemented by someone else
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("TODO: code to draw a select box");
    }
}

fn trait_bound_objects() {
    println!("\n----- 17.2 trait bound objects -----\n");

    let screen = Screen {
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
    screen.run();
}

fn main() {
    let collection = AveragedCollection::new();
    println!("this is an averaged collection:\n\n{:#?}", collection);

    trait_bound_objects();
}
