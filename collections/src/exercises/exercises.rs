pub mod exercises {
    pub fn average(integers: Vec<i32>) -> f32 {
        let mut total = 0;
        for integer in integers.iter() {
            total += integer;
        }
        total = total as f32;
        let length = integers.len() as f32;
        total / integers.len()
    }

    pub fn pig_latin(strings: Vec<T>) {
    }
}
