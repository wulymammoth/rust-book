pub fn main() {
    println!("\n-- 19.5 macros\n");
    declarative_macros();
    procedural_macros();
}

fn declarative_macros() {
    println!("\n19.5 : declarative macros\n");
    // use of `macro_rules!` construct to define "declarative macros"
    // our implemention of the `vec!` macro
    #[macro_export] // this annotation indicates that this needs to be brought into scope to be used
    macro_rules! vec {
        // this is ONE ARM
        // - `$x:expr` matches any expression
        // - `,*` states that zero or more comma literals may follow
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
}

fn procedural_macros()  {
    println!("\n19.5 : procedural macros\n");
    println!("\nthere are 3 kinds of procedural macros: custom derive, attribute-like, function-like\n");

    use hello_macro::HelloMacro;

    struct Pancakes;

    impl HelloMacro for Pancakes {
        println!("Hello, Macro! My name is Pancakes!");
    }

    Pancakes::hello_macro();
}
