pub fn unsafe_rust() {
    println!("\n-- 19.1 unsafe rust\n");
    dereferencing_a_raw_pointer();
    calling_an_unsafe_function_or_method();
    creating_abstration_over_unsafe_code();
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
fn creating_abstration_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // an attempt at implementing the mutable split function
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        assert!(mid <= len);
        // Rust doesn't understand that we're borrowing different parts of the slice; only that
        // we're borrowing from the same slice twice
        (&mut slice[..mid], &mut slice[mid..])
    }
}
