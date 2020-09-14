use std::{collections::HashMap, thread, time::Duration};

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Today, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn closure() {
    let x = 4;
    let equal_to_x = |z| z == x; // captures (dynamic) environment
    let y = 4;
    assert!(equal_to_x(y));
}

fn move_closure() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    //println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    println!("closure example - captures its environment");
    closure();

    println!("move closure example");
    move_closure();
}

#[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn call_with_different_values() {
         let mut c = Cacher::new(|a| a);
         let v1 = c.value(1);
         let v2 = c.value(2);

         assert_eq!(v1, 1);
         assert_eq!(v2, 2);
     }
}
