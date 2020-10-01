use oop::{AveragedCollection, Draw, Post, Screen};

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
#[allow(dead_code)]
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
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}

fn state_pattern() {
    println!("\n----- 17.3 OOP : implementing an object-oriented design pattern (state pattern)\n");

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());
}

fn encoding_states_and_behavior_as_types() {
    println!("\n----- 17.3 OOP : implementing an object-oriented design pattern (encoding states and behavior types)\n");

    use oop::redux;

    let mut post = redux::Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

fn main() {
    let collection = AveragedCollection::new();
    println!("this is an averaged collection:\n{:#?}", collection);

    trait_bound_objects();
    state_pattern();
    encoding_states_and_behavior_as_types();
}
