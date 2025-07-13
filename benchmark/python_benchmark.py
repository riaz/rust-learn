import time
import math
import sys
from typing import List

sys.set_int_max_str_digits(0)

def is_prime(n: int) -> bool:
    """Check if a number is prime"""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    
    # If none of the numbers between 3 and sqrt(n)+1
    # is a factor its a prime number
    for i in range(3, int(math.sqrt(n))+ 1, 2):
        if n % i == 0:
            return False
    return True

def find_primes_in_range(start: int, end: int) -> List[int]:
    """Find all prime numbers in range"""
    results = []
    for num in range(start, end+1):
        if is_prime(num):
            results.append(num)
    return results

def calculate_prime_sum(limit: int) -> int:
    """calculate the sum of all promes upto limit"""
    total = 0
    for num in range(2, limit+1):
        if is_prime(num):
            total += num
    return total

def fibonacci_optimized(n: int) -> int:
    """Just returns the nth fibinacci number
       using a loop
    """
    if n <= 1:
        return n
    
    a, b = 0, 1
    for _ in range(2, n+1):
        a, b = b, a + b

    return b

def matrix_multiplication(size: int) -> float:
    """Perform matrix multiplication benchmark"""
    # Create two matrices filled with random-like values
    matrix_a = [[float(i * j + i + j) for j in range(size)] for i in range(size)]
    matrix_b = [[float(i + j + 1) for j in range(size)] for i in range(size)]
    
    # Multiply matrices
    result = [[0.0 for _ in range(size)] for _ in range(size)]
    for i in range(size):
        for j in range(size):
            for k in range(size):
                result[i][j] += matrix_a[i][k] * matrix_b[k][j]
    
    # Return sum of all elements as a simple result
    return sum(sum(row) for row in result)

def run_benchmarks():
    """Run all benchmarks and measure execution time"""
    print("=== Python Performance Benchmarks ===\n")
    
    # Prime calculation benchmark
    print("1. Prime Calculation Benchmark:")
    start_time = time.time()
    prime_sum = calculate_prime_sum(50000)
    prime_time = time.time() - start_time
    print(f"   Sum of primes up to 50,000: {prime_sum}")
    print(f"   Time taken: {prime_time:.4f} seconds\n")
    
    # Fibonacci benchmark
    print("2. Fibonacci Calculation Benchmark:")
    start_time = time.time()
    fib_result = fibonacci_optimized(1000000)
    fib_time = time.time() - start_time
    print(f"   1,000,000th Fibonacci number (last 10 digits): {str(fib_result)[-10:]}")
    print(f"   Time taken: {fib_time:.4f} seconds\n")
    
    # Matrix multiplication benchmark
    print("3. Matrix Multiplication Benchmark:")
    start_time = time.time()
    matrix_result = matrix_multiplication(300)
    matrix_time = time.time() - start_time
    print(f"   300x300 matrix multiplication result sum: {matrix_result:.2f}")
    print(f"   Time taken: {matrix_time:.4f} seconds\n")
    
    # Find primes in range
    print("4. Prime Range Finding Benchmark:")
    start_time = time.time()
    primes_in_range = find_primes_in_range(10000, 11000)
    range_time = time.time() - start_time
    print(f"   Primes between 10,000 and 11,000: {len(primes_in_range)} found")
    print(f"   Time taken: {range_time:.4f} seconds\n")
    
    total_time = prime_time + fib_time + matrix_time + range_time
    print(f"=== Total Python Execution Time: {total_time:.4f} seconds ===")

if __name__ == "__main__":
    run_benchmarks()
