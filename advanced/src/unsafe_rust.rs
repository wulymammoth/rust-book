pub fn main() {
    println!("\n-- 19.1 unsafe rust\n");
    dereferencing_a_raw_pointer();
    calling_an_unsafe_function_or_method();
    creating_abstration_over_unsafe_code();
    extern_functions();
    accessing_or_modifying_mutable_static_vars();
    unsafe_trait();
}

/// raw pointers
///
/// - are allowed to ignore the  borrowing rules by having both immutable and mutable pointers or
/// multiple mutable pointers to the same locations
/// - arnt't guaranteed to point to valid memory
/// - are allowed to be null
/// - don't implement any automatic cleanup
///
/// reasons to use unsafe rust, trading guaranteed safety in exchange for:
/// - greater performance
/// - interfacing with another language
/// - interfacing with hardware
fn dereferencing_a_raw_pointer() {
    println!("\n- creating raw pointers using `as`");
    let mut num = 5;
    // the * aren't dereference operators. they're part of the type name
    // we use `as` here to cast an immutable and a mutable reference into their corresponding raw
    // pointer types
    let _raw_pointer_1 = &num as *const i32;
    let _raw_pointer_2 = &mut num as *mut i32;

    println!("\n- raw pointer whose validity we aren't assured of");
    // raw pinter to an arbitrary location in memory
    // - possible, but no good reason to write code like this
    let address = 0x912345usize;
    let _raw_pointer_3 = address as *const i32;

    println!("\n- raw pointer that can't be deref'ed unless marked unsafe");
    let mut num = 5;
    // creating pointers is harmless
    let raw_pointer_4 = &num as *const i32;
    let raw_pointer_5 = &mut num as *mut i32;
    unsafe {
        // it's usually accessing the data that can be harmful
        println!("r4 is: {}", *raw_pointer_4);
        println!("r5 is: {}", *raw_pointer_5);
    }
}

/// calling an unsafe function or method
fn calling_an_unsafe_function_or_method() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

/// creating safe abstraction over unsafe code
use std::slice;

fn creating_abstration_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // an attempt at implementing the mutable split function
    #[allow(dead_code)]
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr(); // access the raw pointer of a slice => *mut i32

        assert!(mid <= len); // panics if out of bound
                             // Rust doesn't understand that we're borrowing different parts of the slice; only that
                             // we're borrowing from the same slice twice
        unsafe {
            // slice could be invalid
            (
                slice::from_raw_parts_mut(ptr, mid), // assumes and trusts that the pointer (slice) is valid
                slice::from_raw_parts_mut(ptr.add(mid), len - mid), // add doesn't work if slice is invalid
            )
        }
    }
}

fn extern_functions() {
    extern "C" {
        // defines which ABI (application binary interface)
        fn abs(input: i32) -> i32;
    }

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    unsafe {
        println!("Absolute value of =3 according to C: {}", abs(-3));
    }
}

/// In Rust, global vars are "static variables"
/// - supported, but can be problematic with ownership rules
/// - if two threads are accessing the same mutable global var, a data race can occur
/// * values in static variables have a fixed address in memory, whereas that of a constant is
/// permitted to be duplicated whenever used
/// * static vars can be mutable and constants cannot - accessing and modifying mutable static vars
/// is unsafe
fn accessing_or_modifying_mutable_static_vars() {
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    // access and modifying a mutable static vvariable
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            // any code that modifies COUNTER must be wrapped in unsafe
            COUNTER += inc;
        }
    }

    add_to_count(3);
    unsafe {
        // any code that modifies COUNTER must be wrapped in unsafe
        println!("COUNTER: {}", COUNTER);
    }
}

/// unsafe trait
/// - a trait is unsafe when at least one of its methods has some invariant that the compiler
/// cannot verify
fn unsafe_trait() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
