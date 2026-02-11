# Getting Started with NeetCode 250 Rust

Welcome to your NeetCode 250 Rust DSA practice repository! This guide will help you get started with solving problems.

## Quick Start

### 1. Verify Installation

```bash
# Build everything (should complete without errors)
cargo build --workspace

# Run all tests (all will pass with todo!() - they're templates)
cargo test --workspace
```

### 2. Pick Your First Problem

Start with the Arrays & Hashing category. Here's a good progression:

1. **#1 - Concatenation of Array** (Easy)
2. **#2 - Contains Duplicate** (Easy)
3. **#3 - Valid Anagram** (Easy)
4. **#4 - Two Sum** (Easy)

### 3. Solve a Problem

Let's solve problem #1 as an example:

#### Step 1: Open the problem file
```bash
cd neetcode-250-rust
# Open 01_arrays_and_hashing/src/p001_concatenation_of_array.rs in your editor
```

#### Step 2: Read the problem description
- Check the LeetCode link in the file header
- Read the problem description and examples
- Understand the constraints

#### Step 3: Replace `todo!()` with your solution
```rust
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    // Your solution here
    let mut result = nums.clone();
    result.extend_from_slice(&nums);
    result
}
```

#### Step 4: Add test cases
```rust
#[test]
fn test_example_1() {
    let result = get_concatenation(vec![1, 2, 1]);
    assert_eq!(result, vec![1, 2, 1, 1, 2, 1]);
}
```

#### Step 5: Run tests
```bash
# Test just this problem
cargo test --package p01_arrays_and_hashing p001

# Or test with cargo test and problem name
cargo test --package p01_arrays_and_hashing concatenation
```

#### Step 6: Run with example (optional)
```bash
# Edit the example file to add test inputs
# Then run:
cargo run --package p01_arrays_and_hashing --example p001_concatenation_of_array
```

## Testing Strategies

### Test a Single Problem
```bash
cargo test --package <category> p<number>
# Example:
cargo test --package p01_arrays_and_hashing p001
cargo test --package p07_trees p091
```

### Test by Problem Name
```bash
cargo test --package <category> <keyword>
# Example:
cargo test --package p01_arrays_and_hashing concatenation
cargo test --package p02_two_pointers palindrome
```

### Test Entire Category
```bash
cargo test --package <category>
# Example:
cargo test --package p01_arrays_and_hashing
cargo test --package p05_binary_search
```

### Test Everything
```bash
cargo test --workspace
```

### Run with Examples
```bash
cargo run --package <category> --example p<number>_<slug>
# Example:
cargo run --package p01_arrays_and_hashing --example p001_concatenation_of_array
```

## Category Overview

### Recommended Learning Order

1. **Arrays & Hashing** (22 problems) - Start here!
   - Fundamental data structures
   - Hash maps, arrays, strings

2. **Two Pointers** (13 problems)
   - Efficient array traversal techniques

3. **Sliding Window** (9 problems)
   - Substring and subarray problems

4. **Binary Search** (14 problems)
   - Efficient searching algorithms

5. **Stack** (15 problems)
   - LIFO data structure applications

6. **Linked List** (14 problems)
   - Pointer manipulation

7. **Trees** (23 problems)
   - Binary trees, BST, traversals

8. **Heap / Priority Queue** (12 problems)
   - Top-K problems, scheduling

9. **Graphs** (21 problems)
   - DFS, BFS, graph traversal

10. **Backtracking** (16 problems)
    - Recursive problem solving

11. **Tries** (4 problems)
    - Prefix trees

12. **Advanced Graphs** (10 problems)
    - Dijkstra, Prim's, Kruskal's

13. **1-D Dynamic Programming** (17 problems)
    - Memoization and tabulation

14. **2-D Dynamic Programming** (16 problems)
    - Grid-based DP

15. **Greedy** (14 problems)
    - Optimal substructure

16. **Intervals** (7 problems)
    - Merging, overlapping intervals

17. **Math & Geometry** (13 problems)
    - Mathematical algorithms

18. **Bit Manipulation** (10 problems)
    - Bitwise operations

## Tips for Success

### 1. Start Simple
- Begin with Easy problems
- Don't skip to Medium/Hard too quickly
- Master the fundamentals first

### 2. Use the Test-Driven Approach
- Add test cases before implementing
- Start with example test cases from LeetCode
- Add edge cases as you discover them

### 3. Document Your Approach
- Fill in the "Approach" section in the file
- Add time and space complexity
- Write comments for tricky parts

### 4. Common Rust Patterns

#### Working with Vectors
```rust
// Create
let mut nums = vec![1, 2, 3];

// Iterate
for num in &nums { }
for (i, num) in nums.iter().enumerate() { }

// Modify
nums.push(4);
nums.pop();
```

#### Using HashMap
```rust
use std::collections::HashMap;

let mut map: HashMap<i32, i32> = HashMap::new();
map.insert(key, value);
if let Some(&val) = map.get(&key) { }
```

#### Working with Trees
```rust
use super::{TreeNode, Rc, RefCell};

// Access node value
if let Some(node) = root {
    let val = node.borrow().val;
    let left = node.borrow().left.clone();
    let right = node.borrow().right.clone();
}
```

#### Working with Linked Lists
```rust
use super::ListNode;

// Traverse list
let mut current = head;
while let Some(node) = current {
    let val = node.val;
    current = node.next;
}
```

### 5. When You're Stuck

1. Re-read the problem carefully
2. Work through examples by hand
3. Check the LeetCode discussion section
4. Watch NeetCode video explanation
5. Implement a brute force solution first
6. Optimize from there

## Tracking Your Progress

The main README.md contains a checkbox list of all 250 problems. Update it as you complete problems:

```markdown
- [x] **001**. [Concatenation of Array](https://leetcode.com/...) - *Easy*
- [ ] **002**. [Contains Duplicate](https://leetcode.com/...) - *Easy*
```

## Useful Commands Reference

```bash
# Build
cargo build --workspace              # Build all
cargo build --package <category>     # Build one category

# Test
cargo test --workspace               # Test all
cargo test --package <category>      # Test category
cargo test --package <cat> p001      # Test specific problem
cargo test <keyword>                 # Search by keyword

# Run Examples
cargo run --package <cat> --example p001_<slug>

# Check (faster than build)
cargo check --workspace

# Clean
cargo clean
```

## IDE Setup

### VS Code with rust-analyzer

1. Install the `rust-analyzer` extension
2. Open the workspace root folder
3. Click "Run Test" or "Debug" above any test function
4. Tests run with full IDE integration

### IntelliJ IDEA / CLion

1. Install Rust plugin
2. Import as Cargo project
3. Right-click on test functions to run

## Getting Help

- **NeetCode Videos**: https://neetcode.io/
- **LeetCode Discuss**: Check each problem's discussion tab
- **Rust Documentation**: https://doc.rust-lang.org/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/

## Next Steps

1. Build the workspace to verify everything works
2. Pick your first problem from Arrays & Hashing
3. Read the problem on LeetCode
4. Implement your solution
5. Add test cases
6. Run tests
7. Mark it complete in README.md
8. Move to the next problem!

Happy coding! 🦀
