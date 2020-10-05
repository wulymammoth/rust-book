pub fn main() {
    println!("\n-- 19.1 advanced traits\n");
    specifying_placeholder_types_in_trait_defs_with_associated_types();
}

fn specifying_placeholder_types_in_trait_defs_with_associated_types() {
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }
}
