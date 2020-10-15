mod advanced_traits;
mod advanced_types;
mod unsafe_rust;

fn main() {
    println!("===== 19. advanced features ======\n");
    unsafe_rust::main();
    advanced_traits::main();
    advanced_types::main();
}
