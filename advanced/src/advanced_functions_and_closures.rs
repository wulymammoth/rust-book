pub fn main() {
    println!("\n-- 19.4 advanced functions and closures\n");
    println!("\n-- 19.5 macros\n");
    macro_rules();
}

fn macro_rules() {
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
