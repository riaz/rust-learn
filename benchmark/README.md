## Performance Benchmark

This project is to benchmark Pure-Python vs Pure Rust vs Rust Code with Python Binding

## Directory Structure

benchmark/
├── src/
│   └── lib.rs          # Rust library with Python bindings
├── Cargo.toml          # Rust dependencies and build config
├── python_benchmark.py # Pure Python implementation
├── rust_benchmark.rs   # Pure Rust implementation  
└── comparison.py       # Python script using Rust backend