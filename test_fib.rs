//! Standalone test for Fibonacci.
//! Compile with: verus test_fib.rs --extern lib=liblib.rlib --compile

extern crate lib;

use lib::Chap11::FibonacciStEph::FibonacciStEph::fib;

fn main() {
    println!("Testing Fibonacci...");
    
    let known: [(u64, u64); 7] = [
        (0, 0), (1, 1), (2, 1), (5, 5), (10, 55), (20, 6765), (30, 832040),
    ];
    
    let mut all_passed = true;
    for (n, expected) in known {
        let result = fib(n);
        if result == expected {
            println!("  fib({}) = {} ✓", n, result);
        } else {
            println!("  fib({}) = {} ✗ (expected {})", n, result, expected);
            all_passed = false;
        }
    }
    
    if all_passed {
        println!("\nAll tests passed!");
    } else {
        println!("\nSome tests failed!");
        std::process::exit(1);
    }
}

