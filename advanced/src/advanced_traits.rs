pub fn main() {
    println!("\n-- 19.1 advanced traits\n");
    specifying_placeholder_types_in_trait_defs_with_associated_types();
}

fn default_generic_type_parameters_and_operator_overloading() {}

fn specifying_placeholder_types_in_trait_defs_with_associated_types() {
    pub trait Iterator {
        type Item; // placeholder type

        // implementors of Iterator must specify a concrete type for Item
        // NOTE: with associated types, we don't need to annotate types because we can't implement
        // a trait on a type multiple times
        fn next(&mut self) -> Option<Self::Item>;
    }
}
