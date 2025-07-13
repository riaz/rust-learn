use std::time::Instant;
use num_bigint::BigUint;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    // Check odd divisors up to sqrt(n)
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_primes_in_range(start: u64, end: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    for num in start..=end {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

fn calculate_prime_sum(limit: u64) -> u64 {
    let mut total = 0;
    for num in 2..=limit {
        if is_prime(num) {
            total += num;
        }
    }
    total
}

fn fibonacci_optimized(n: u64) -> BigUint {
    if n <= 1 {
        return BigUint::from(n);
    }
    
    let mut a = BigUint::from(0u32);
    let mut b = BigUint::from(1u32);
    
    for _ in 2..=n {
        let temp = &a + &b;
        a = std::mem::replace(&mut b, temp);
    }
    b
}

fn matrix_multiplication(size: usize) -> f64 {
    // Create two matrices filled with values
    let mut matrix_a = vec![vec![0.0f64; size]; size];
    let mut matrix_b = vec![vec![0.0f64; size]; size];
    
    // Fill matrices with deterministic values
    for i in 0..size {
        for j in 0..size {
            matrix_a[i][j] = (i * j + i + j) as f64;
            matrix_b[i][j] = (i + j + 1) as f64;
        }
    }
    
    // Multiply matrices
    let mut result = vec![vec![0.0f64; size]; size];
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                result[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }
    
    // Return sum of all elements
    result.iter().map(|row| row.iter().sum::<f64>()).sum()
}

fn run_benchmarks() {
    println!("=== Rust Performance Benchmarks ===\n");
    
    // Prime calculation benchmark
    println!("1. Prime Calculation Benchmark:");
    let start = Instant::now();
    let prime_sum = calculate_prime_sum(50000);
    let prime_time = start.elapsed();
    println!("   Sum of primes up to 50,000: {}", prime_sum);
    println!("   Time taken: {:.4} seconds\n", prime_time.as_secs_f64());
    
    // Fibonacci benchmark
    println!("2. Fibonacci Calculation Benchmark:");
    let start = Instant::now();
    
    let fib_result = fibonacci_optimized(1000);

    let fib_time = start.elapsed();
    let fib_str = fib_result.to_string();
    let last_10 = if fib_str.len() >= 10 {
        &fib_str[fib_str.len()-10..]
    } else {
        &fib_str
    };
    println!("   1,000,000th Fibonacci number (last 10 digits): {}", last_10);
    println!("   Time taken: {:.4} seconds\n", fib_time.as_secs_f64());
    
    // Matrix multiplication benchmark
    println!("3. Matrix Multiplication Benchmark:");
    let start = Instant::now();
    let matrix_result = matrix_multiplication(300);
    let matrix_time = start.elapsed();
    println!("   300x300 matrix multiplication result sum: {:.2}", matrix_result);
    println!("   Time taken: {:.4} seconds\n", matrix_time.as_secs_f64());
    
    // Find primes in range
    println!("4. Prime Range Finding Benchmark:");
    let start = Instant::now();
    let primes_in_range = find_primes_in_range(10000, 11000);
    let range_time = start.elapsed();
    println!("   Primes between 10,000 and 11,000: {} found", primes_in_range.len());
    println!("   Time taken: {:.4} seconds\n", range_time.as_secs_f64());
    
    let total_time = prime_time.as_secs_f64() + fib_time.as_secs_f64() + 
                     matrix_time.as_secs_f64() + range_time.as_secs_f64();
    println!("=== Total Rust Execution Time: {:.4} seconds ===", total_time);
}

fn main() {
    run_benchmarks();
}