use std::cmp::max;

fn largest_item(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        largest = max(largest, item);
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        largest = max(largest, item);
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        largest = max(largest, item);
    }
    largest
}

//fn gen_largest<T>(list: &[T]) -> T {
//let mut largest = list[0];
//for &item in list {
//// note: `T` might need a bound for `std::cmp::PartialOrd`
//// - this means that it can't possibly work for all possible types of T
//// - can only use types whose values can be ordered
//// - will need to either limit the types or use "traits as parameters"
//if item > largest {
//largest = item;
//}
//}
//largest
//}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pnt<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pnt<T, U> {
    fn mixup<V, W>(self, other: Pnt<V, W>) -> Pnt<T, W> {
        Pnt {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn gen_data_types() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for &number in &number_list {
        largest = max(largest, number);
    }

    println!("the largest number is {}; expected to be 100", largest);
    println!(
        "the largest number using our function is {}; expected to be 100",
        largest_item(&number_list)
    );

    // duplicated just to operate on another list
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for &number in &number_list {
        largest = max(largest, number);
    }

    println!("the largest number is {}; expected to be 6000", largest);
    println!(
        "the largest number using our function is {}; expected to be 100",
        largest_item(&number_list)
    );

    // duplication for demonstration purposes
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("the largest number is {}; expected to be 100", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("the largest char is {}; expected to be 'y'", result);

    // generic point and method
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p: {:?}", p);
    let p = Point { x: 5.0, y: 10.0 };
    println!("distance from origin {}", p.distance_from_origin());

    // mixing up types
    let p1 = Pnt { x: 5, y: 10.4 };
    let p2 = Pnt { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
