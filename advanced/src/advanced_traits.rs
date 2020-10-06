pub fn main() {
    println!("\n-- 19.2 advanced traits\n");
    specifying_placeholder_types_in_trait_defs_with_associated_types();
    default_generic_type_parameters_and_operator_overloading();
    fully_qualified_syntax_for_disambiguation();
}

/// fully qualified syntax for disambiguation: calling methods with the same name
fn fully_qualified_syntax_for_disambiguation() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*flapping arms furiously*");
        }
    }

    let person = Human;
    // specifying trait name specifies which implementation we want
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // Human::fly

    // NOTE: associated functions that are a part of traits don't have a `self` parameter
    println!("\n- trait associated functions\n");
    trait Animal {
        fn baby_name() -> String; // associated function (no self)
    }

    struct Dog;

    #[allow(dead_code)]
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // use of fully-qualified name for an associated function with no self param
    // NOTE: no receiver for associated functions, only other arguments
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

fn default_generic_type_parameters_and_operator_overloading() {
    println!("\n- default generic type parameters and operator overloading\n");
    // operator overloading : customizing the behavior of an operator such as `+`
    // - we can overload the operations corresponding to traits listed in `std::ops`

    // we overload the `+` to add two `Point` instances together
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let composite_point = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
    println!(
        "\n- overloading the `+` operator | adding two points: \nfirst point: {:?}\nsecond point: {:?}\ncomposite: {:?}\n",
        Point { x: 1, y: 0 },
        Point { x: 2, y: 3 },
        composite_point,
    );
    assert_eq!(composite_point, Point { x: 3, y: 3 });

    // customized default type parameters
    //
    // NOTE: 2 primary uses for default type params
    // 1. extend a type without breaking existing code
    // 2. allow customization in specific cases where most users won't need
    // * standard library Add trait is an example of customiztion in specific use cases where we're
    // adding two similar types
    //
    // - implementing Add trait where we customize `Rhs`
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

fn specifying_placeholder_types_in_trait_defs_with_associated_types() {
    println!("\n- specifying placeholder types in trait definitions with associated types\n");
    pub trait Iterator {
        type Item; // placeholder type

        // implementors of Iterator must specify a concrete type for Item
        // NOTE: with associated types, we don't need to annotate types because we can't implement
        // a trait on a type multiple times
        fn next(&mut self) -> Option<Self::Item>;
    }
}
