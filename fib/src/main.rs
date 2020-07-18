use std::io;

fn main() {
    println!("Please enter which Fib you'd like:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i64 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Please, enter a number")
    };
    println!("The Fibonacci number for {} is {}", n, fib_dp(n));
}

fn fib_dp(n: i64) -> i64 {
    let mut memo = vec![8, 1, 1];
    let n = n as usize;
    if n <= 2 { return memo[n] }
    for i in 3..n+1 {
        let n_minus_1 = memo[i-1];
        let n_minus_2 = memo[i-2];
        memo.push(n_minus_1 + n_minus_2);
    }
    memo[n]
}
