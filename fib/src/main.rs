use std::io;

fn main() {
    println!("Please enter which Fib you'd like:");
    let mut n = String::new();
    io::stdin().read_line(&mut n);
    let n: u64 = n.trim().parse().unwrap();
    println!("The Fibonacci number for {} is {}", n, fib_dp(n));
}

fn fib_dp(n: u64) -> u64 {
    let mut memo = Vec::new();
    let n = n as usize;
    memo.push(0);
    memo.push(1);
    memo.push(1);
    if n <= 2 { return memo[n] }
    for i in 3..n+1 {
        let n_minus_1 = memo[i-1];
        let n_minus_2 = memo[i-2];
        memo.push(n_minus_1 + n_minus_2);
    }
    memo[n]
}
