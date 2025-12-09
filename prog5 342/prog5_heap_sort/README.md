# Heap Sort Implementation - README

## Overview
This project implements an efficient **Heap Sort** algorithm in Rust with O(n log n) time complexity and O(1) space complexity (in-place sorting).

## Features
- **Two implementations**:
  - Recursive heapify (standard approach)
  - Iterative heapify (optimized for performance)
- **Generic implementation**: Works with any type that implements `Ord`
- **Comprehensive test suite**: 7 unit tests covering edge cases
- **Performance benchmarking**: Included timing tests

## How to Compile and Run

### Prerequisites
Install Rust from [https://rustup.rs/](https://rustup.rs/)

### Compile and Run
```bash
cd "C:\Users\17143\.gemini\antigravity\scratch\assignment\prog5 342\prog5_heap_sort"
cargo build --release
cargo run --release
```

### Run Tests
```bash
cargo test
```

## Algorithm Explanation

### Heap Sort Process
1. **Build Max Heap**: Rearrange the array into a max heap structure
2. **Extract Elements**: Repeatedly swap the root (maximum) with the last element and heapify

### Time Complexity
- **Best Case**: O(n log n)
- **Average Case**: O(n log n)
- **Worst Case**: O(n log n)

### Space Complexity
- **O(1)**: In-place sorting, no additional arrays needed

## Code Structure

- `heap_sort()`: Standard recursive heap sort
- `heap_sort_optimized()`: Iterative version for better performance
- `heapify()`: Recursive heapify function
- `heapify_iterative()`: Iterative heapify (tail-call optimized)
- Comprehensive test suite and examples

## Installation Instructions

Since Rust is not currently installed on your system, here's how to get it:

1. **Install Rust**:
   ```powershell
   # Download and run rustup-init.exe from https://rustup.rs/
   # Or use PowerShell:
   Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
   & "$env:TEMP\rustup-init.exe"
   ```

2. **Verify installation**:
   ```powershell
   rustc --version
   cargo --version
   ```

3. **Compile this project**:
   ```powershell
   cd "C:\Users\17143\.gemini\antigravity\scratch\assignment\prog5 342\prog5_heap_sort"
   cargo build --release
   cargo run --release
   ```
