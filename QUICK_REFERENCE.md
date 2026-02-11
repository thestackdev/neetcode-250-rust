# Quick Reference

## Essential Commands

```bash
# Build everything
cargo build --workspace

# Test specific problem
cargo test --package p01_arrays_and_hashing p001

# Test by keyword
cargo test --package p07_trees invert

# Test entire category
cargo test --package p05_binary_search

# Run example
cargo run --package p04_stack --example p045_baseball_game
```

## File Locations

```
Problem files:     <category>/src/p{number}_{slug}.rs
Example runners:   <category>/examples/p{number}_{slug}.rs
Library file:      <category>/src/lib.rs
Category config:   <category>/Cargo.toml
```

## Common Rust Patterns

### HashMap
```rust
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert(key, value);
if let Some(&v) = map.get(&key) { }
```

### HashSet
```rust
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert(value);
if set.contains(&value) { }
```

### Vector Operations
```rust
let mut v = vec![1, 2, 3];
v.push(4);
v.pop();
v.sort();
v.reverse();
v.iter().sum::<i32>()
```

### String Operations
```rust
let s = String::from("hello");
s.chars().collect::<Vec<char>>()
s.len()
s.is_empty()
&s[0..3]  // substring
```

### Tree Navigation
```rust
use super::{TreeNode, Rc, RefCell};

if let Some(node) = root {
    let val = node.borrow().val;
    let left = node.borrow().left.clone();
    let right = node.borrow().right.clone();
}
```

### List Navigation
```rust
use super::ListNode;

let mut curr = head;
while let Some(node) = curr {
    println!("{}", node.val);
    curr = node.next;
}
```

## Problem-Solving Template

1. Read problem on LeetCode
2. Understand examples and constraints
3. Plan approach (write it in the file!)
4. Write test cases first
5. Implement solution
6. Run: `cargo test --package <cat> p<num>`
7. Refine until tests pass
8. Document complexity

## Category Shortcuts

```bash
# Quick category package names
ah="p01_arrays_and_hashing"
tp="p02_two_pointers"
sw="p03_sliding_window"
st="p04_stack"
bs="p05_binary_search"
ll="p06_linked_list"
tr="p07_trees"
hp="p08_heap_priority_queue"
bt="p09_backtracking"
ti="p10_tries"
gr="p11_graphs"
ag="p12_advanced_graphs"
d1="p13_dp_1d"
d2="p14_dp_2d"
gd="p15_greedy"
iv="p16_intervals"
mg="p17_math_geometry"
bm="p18_bit_manipulation"

# Usage:
cargo test --package $ah p001
```

## Difficulty Distribution

- **Easy**: ~90 problems - Start here!
- **Medium**: ~120 problems - Core practice
- **Hard**: ~40 problems - Advanced challenges

## When Stuck

1. Brute force first (get it working)
2. Identify bottlenecks
3. Optimize data structures
4. Consider time/space tradeoffs
5. Check LeetCode hints
6. Watch NeetCode video

## Progress Tracking

Update README.md as you complete problems:
```markdown
- [x] **001**. Problem Name - *Easy* ✅
```

## IDE Integration

### VS Code
- Install `rust-analyzer`
- Click "Run Test" above test functions
- Use CodeLens features

### Vim/Neovim
- Install `rust-analyzer` LSP
- Use `:RustTest` or similar commands

## Performance Tips

```bash
# Check without building (faster)
cargo check --workspace

# Build in release mode (optimized)
cargo build --release

# Run tests in release mode
cargo test --release

# Clean build cache
cargo clean
```

## Workspace Structure

```
neetcode-250-rust/
├── Cargo.toml              (workspace root)
├── README.md               (progress tracker)
├── GETTING_STARTED.md      (detailed guide)
├── QUICK_REFERENCE.md      (this file)
├── 01_arrays_and_hashing/     (22 problems)
├── 02_two_pointers/           (13 problems)
├── 03_sliding_window/         (9 problems)
├── 04_stack/                  (15 problems)
├── 05_binary_search/          (14 problems)
├── 06_linked_list/            (14 problems)
├── 07_trees/                  (23 problems)
├── 08_heap_priority_queue/    (12 problems)
├── 09_backtracking/           (16 problems)
├── 10_tries/                  (4 problems)
├── 11_graphs/                 (21 problems)
├── advanced_11_graphs/        (10 problems)
├── 13_dp_1d/                  (17 problems)
├── 14_dp_2d/                  (16 problems)
├── 15_greedy/                 (14 problems)
├── 16_intervals/              (7 problems)
├── 17_math_geometry/          (13 problems)
└── 18_bit_manipulation/       (10 problems)
```

## Resources

- **NeetCode**: https://neetcode.io/
- **LeetCode**: https://leetcode.com/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rust std docs**: https://doc.rust-lang.org/std/

---

Happy coding! 🦀
