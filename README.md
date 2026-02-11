# NeetCode 250 - Rust DSA Practice

A comprehensive Rust workspace containing all 250 problems from the NeetCode 250 list, organized by category with starter templates and test cases.

## Overview

This repository provides a structured environment for practicing Data Structures and Algorithms problems in Rust. Each problem includes:
- Complete problem description and examples
- LeetCode problem link
- Function signature with proper Rust types
- Test case templates
- Initial `todo!()` implementation for you to complete

## Structure

The workspace is organized into 18 category-based crates:

- **01. Arrays And Hashing** (22 problems)
- **02. Two Pointers** (13 problems)
- **03. Sliding Window** (9 problems)
- **04. Stack** (15 problems)
- **05. Binary Search** (14 problems)
- **06. Linked List** (14 problems)
- **07. Trees** (23 problems)
- **08. Heap Priority Queue** (12 problems)
- **09. Backtracking** (16 problems)
- **10. Tries** (4 problems)
- **11. Graphs** (21 problems)
- **12. Advanced Graphs** (10 problems)
- **13. Dp 1D** (17 problems)
- **14. Dp 2D** (16 problems)
- **15. Greedy** (14 problems)
- **16. Intervals** (7 problems)
- **17. Math Geometry** (13 problems)
- **18. Bit Manipulation** (10 problems)

## Setup

### Prerequisites
- Rust toolchain (1.70.0 or later)
- Cargo

### Build All
```bash
cargo build --workspace
```

### Test All
```bash
cargo test --workspace
```

## How to Test Individual Solutions

### 1. Test a Specific Problem
```bash
# From workspace root
cargo test --package p01_arrays_and_hashing p001

# Or with partial name match
cargo test --package p01_arrays_and_hashing concatenation
```

### 2. Test All Problems in a Category
```bash
cargo test --package p01_arrays_and_hashing
```

### 3. Test Everything
```bash
cargo test --workspace
```

### 4. Run a Single Problem with main()
Uncomment the `main()` function in any problem file and run:
```bash
cd 01_arrays_and_hashing
cargo run --example p001_concatenation_of_array
```

### 5. Use Rust Analyzer in VS Code
- Open any problem file
- Click "Run Test" or "Debug" above test functions
- Tests run individually with full IDE integration

## Quick Start

1. Pick a problem from the progress tracker below
2. Navigate to the category directory
3. Open the problem file (e.g., `src/p001_concatenation_of_array.rs`)
4. Replace the `todo!()` with your solution
5. Run tests: `cargo test --package <category> <problem_number>`
6. Iterate until all tests pass!

## Progress Tracker

Track your progress by checking off completed problems:


### Arrays And Hashing

- [ ] **001**. [Concatenation of Array](https://leetcode.com/problems/concatenation-of-array/) - *Easy*
- [ ] **002**. [Contains Duplicate](https://leetcode.com/problems/contains-duplicate/) - *Easy*
- [ ] **003**. [Valid Anagram](https://leetcode.com/problems/valid-anagram/) - *Easy*
- [ ] **004**. [Two Sum](https://leetcode.com/problems/two-sum/) - *Easy*
- [ ] **005**. [Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/) - *Easy*
- [ ] **006**. [Group Anagrams](https://leetcode.com/problems/group-anagrams/) - *Medium*
- [ ] **007**. [Remove Element](https://leetcode.com/problems/remove-element/) - *Easy*
- [ ] **008**. [Majority Element](https://leetcode.com/problems/majority-element/) - *Easy*
- [ ] **009**. [Design HashSet](https://leetcode.com/problems/design-hashset/) - *Easy*
- [ ] **010**. [Design HashMap](https://leetcode.com/problems/design-hashmap/) - *Easy*
- [ ] **011**. [Sort an Array](https://leetcode.com/problems/sort-an-array/) - *Medium*
- [ ] **012**. [Sort Colors](https://leetcode.com/problems/sort-colors/) - *Medium*
- [ ] **013**. [Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/) - *Medium*
- [ ] **014**. [Encode and Decode Strings](https://leetcode.com/problems/encode-and-decode-strings/) - *Medium*
- [ ] **015**. [Range Sum Query 2D Immutable](https://leetcode.com/problems/range-sum-query-2d-immutable/) - *Medium*
- [ ] **016**. [Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/) - *Medium*
- [ ] **017**. [Valid Sudoku](https://leetcode.com/problems/valid-sudoku/) - *Medium*
- [ ] **018**. [Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/) - *Medium*
- [ ] **019**. [Best Time to Buy And Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/) - *Medium*
- [ ] **020**. [Majority Element II](https://leetcode.com/problems/majority-element-ii) - *Medium*
- [ ] **021**. [Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/) - *Medium*
- [ ] **022**. [First Missing Positive](https://leetcode.com/problems/first-missing-positive/) - *Hard*

### Two Pointers

- [ ] **023**. [Reverse String](https://leetcode.com/problems/reverse-string/) - *Easy*
- [ ] **024**. [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/) - *Easy*
- [ ] **025**. [Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii/) - *Easy*
- [ ] **026**. [Merge Strings Alternately](https://leetcode.com/problems/merge-strings-alternately/) - *Easy*
- [ ] **027**. [Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/) - *Easy*
- [ ] **028**. [Remove Duplicates From Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/) - *Easy*
- [ ] **029**. [Two Sum II Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/) - *Medium*
- [ ] **030**. [3Sum](https://leetcode.com/problems/3sum/) - *Medium*
- [ ] **031**. [4Sum](https://leetcode.com/problems/4sum/) - *Medium*
- [ ] **032**. [Rotate Array](https://leetcode.com/problems/rotate-array/) - *Medium*
- [ ] **033**. [Container With Most Water](https://leetcode.com/problems/container-with-most-water/) - *Medium*
- [ ] **034**. [Boats to Save People](https://leetcode.com/problems/boats-to-save-people/) - *Medium*
- [ ] **035**. [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/) - *Hard*

### Sliding Window

- [ ] **036**. [Contains Duplicate II](https://leetcode.com/problems/contains-duplicate-ii/) - *Easy*
- [ ] **037**. [Best Time to Buy And Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/) - *Easy*
- [ ] **038**. [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) - *Medium*
- [ ] **039**. [Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/) - *Medium*
- [ ] **040**. [Permutation In String](https://leetcode.com/problems/permutation-in-string/) - *Medium*
- [ ] **041**. [Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/) - *Medium*
- [ ] **042**. [Find K Closest Elements](https://leetcode.com/problems/find-k-closest-elements/) - *Medium*
- [ ] **043**. [Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/) - *Hard*
- [ ] **044**. [Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/) - *Hard*

### Stack

- [ ] **045**. [Baseball Game](https://leetcode.com/problems/baseball-game/) - *Easy*
- [ ] **046**. [Valid Parentheses](https://leetcode.com/problems/valid-parentheses/) - *Easy*
- [ ] **047**. [Implement Stack Using Queues](https://leetcode.com/problems/implement-stack-using-queues/) - *Easy*
- [ ] **048**. [Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks) - *Easy*
- [ ] **049**. [Min Stack](https://leetcode.com/problems/min-stack/) - *Medium*
- [ ] **050**. [Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/) - *Medium*
- [ ] **051**. [Generate Parentheses](https://leetcode.com/problems/generate-parentheses/) - *Medium*
- [ ] **052**. [Asteroid Collision](https://leetcode.com/problems/asteroid-collision/) - *Medium*
- [ ] **053**. [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/) - *Medium*
- [ ] **054**. [Online Stock Span](https://leetcode.com/problems/online-stock-span/) - *Medium*
- [ ] **055**. [Car Fleet](https://leetcode.com/problems/car-fleet/) - *Medium*
- [ ] **056**. [Simplify Path](https://leetcode.com/problems/simplify-path/) - *Medium*
- [ ] **057**. [Decode String](https://leetcode.com/problems/decode-string/) - *Medium*
- [ ] **058**. [Maximum Frequency Stack](https://leetcode.com/problems/maximum-frequency-stack/) - *Hard*
- [ ] **059**. [Largest Rectangle In Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/) - *Hard*

### Binary Search

- [ ] **060**. [Binary Search](https://leetcode.com/problems/binary-search/) - *Easy*
- [ ] **061**. [Search Insert Position](https://leetcode.com/problems/search-insert-position/) - *Easy*
- [ ] **062**. [Guess Number Higher Or Lower](https://leetcode.com/problems/guess-number-higher-or-lower/) - *Easy*
- [ ] **063**. [Sqrt(x)](https://leetcode.com/problems/sqrtx/) - *Easy*
- [ ] **064**. [Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/) - *Medium*
- [ ] **065**. [Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/) - *Medium*
- [ ] **066**. [Capacity to Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/) - *Medium*
- [ ] **067**. [Find Minimum In Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/) - *Medium*
- [ ] **068**. [Search In Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/) - *Medium*
- [ ] **069**. [Search In Rotated Sorted Array II](https://leetcode.com/problems/search-in-rotated-sorted-array-ii/) - *Medium*
- [ ] **070**. [Time Based Key Value Store](https://leetcode.com/problems/time-based-key-value-store/) - *Medium*
- [ ] **071**. [Split Array Largest Sum](https://leetcode.com/problems/split-array-largest-sum/) - *Hard*
- [ ] **072**. [Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/) - *Hard*
- [ ] **073**. [Find in Mountain Array](https://leetcode.com/problems/find-in-mountain-array) - *Hard*

### Linked List

- [ ] **074**. [Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/) - *Easy*
- [ ] **075**. [Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/) - *Easy*
- [ ] **076**. [Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/) - *Easy*
- [ ] **077**. [Reorder List](https://leetcode.com/problems/reorder-list/) - *Medium*
- [ ] **078**. [Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/) - *Medium*
- [ ] **079**. [Copy List With Random Pointer](https://leetcode.com/problems/copy-list-with-random-pointer/) - *Medium*
- [ ] **080**. [Add Two Numbers](https://leetcode.com/problems/add-two-numbers/) - *Medium*
- [ ] **081**. [Find The Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/) - *Medium*
- [ ] **082**. [Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii/) - *Medium*
- [ ] **083**. [Design Circular Queue](https://leetcode.com/problems/design-circular-queue/) - *Medium*
- [ ] **084**. [LRU Cache](https://leetcode.com/problems/lru-cache/) - *Medium*
- [ ] **085**. [LFU Cache](https://leetcode.com/problems/lfu-cache/) - *Hard*
- [ ] **086**. [Merge K Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/) - *Hard*
- [ ] **087**. [Reverse Nodes In K Group](https://leetcode.com/problems/reverse-nodes-in-k-group/) - *Hard*

### Trees

- [ ] **088**. [Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/) - *Easy*
- [ ] **089**. [Binary Tree Preorder Traversal](https://leetcode.com/problems/binary-tree-preorder-traversal/) - *Easy*
- [ ] **090**. [Binary Tree Postorder Traversal](https://leetcode.com/problems/binary-tree-postorder-traversal/) - *Easy*
- [ ] **091**. [Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/) - *Easy*
- [ ] **092**. [Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/) - *Easy*
- [ ] **093**. [Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/) - *Easy*
- [ ] **094**. [Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/) - *Easy*
- [ ] **095**. [Same Tree](https://leetcode.com/problems/same-tree/) - *Easy*
- [ ] **096**. [Subtree of Another Tree](https://leetcode.com/problems/subtree-of-another-tree/) - *Easy*
- [ ] **097**. [Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/) - *Medium*
- [ ] **098**. [Insert into a Binary Search Tree](https://leetcode.com/problems/insert-into-a-binary-search-tree/) - *Medium*
- [ ] **099**. [Delete Node in a BST](https://leetcode.com/problems/delete-node-in-a-bst/) - *Medium*
- [ ] **100**. [Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/) - *Medium*
- [ ] **101**. [Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/) - *Medium*
- [ ] **102**. [Construct Quad Tree](https://leetcode.com/problems/construct-quad-tree/) - *Medium*
- [ ] **103**. [Count Good Nodes In Binary Tree](https://leetcode.com/problems/count-good-nodes-in-binary-tree/) - *Medium*
- [ ] **104**. [Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/) - *Medium*
- [ ] **105**. [Kth Smallest Element In a Bst](https://leetcode.com/problems/kth-smallest-element-in-a-bst/) - *Medium*
- [ ] **106**. [Construct Binary Tree From Preorder And Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/) - *Medium*
- [ ] **107**. [House Robber III](https://leetcode.com/problems/house-robber-iii/) - *Medium*
- [ ] **108**. [Delete Leaves With a Given Value](https://leetcode.com/problems/delete-leaves-with-a-given-value) - *Medium*
- [ ] **109**. [Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/) - *Hard*
- [ ] **110**. [Serialize And Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/) - *Hard*

### Heap Priority Queue

- [ ] **111**. [Kth Largest Element In a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/) - *Easy*
- [ ] **112**. [Last Stone Weight](https://leetcode.com/problems/last-stone-weight/) - *Easy*
- [ ] **113**. [K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/) - *Medium*
- [ ] **114**. [Kth Largest Element In An Array](https://leetcode.com/problems/kth-largest-element-in-an-array/) - *Medium*
- [ ] **115**. [Task Scheduler](https://leetcode.com/problems/task-scheduler/) - *Medium*
- [ ] **116**. [Design Twitter](https://leetcode.com/problems/design-twitter/) - *Medium*
- [ ] **117**. [Single Threaded CPU](https://leetcode.com/problems/single-threaded-cpu/) - *Medium*
- [ ] **118**. [Reorganize String](https://leetcode.com/problems/reorganize-string/) - *Medium*
- [ ] **119**. [Longest Happy String](https://leetcode.com/problems/longest-happy-string/) - *Medium*
- [ ] **120**. [Car Pooling](https://leetcode.com/problems/car-pooling/) - *Medium*
- [ ] **121**. [Find Median From Data Stream](https://leetcode.com/problems/find-median-from-data-stream/) - *Hard*
- [ ] **122**. [IPO](https://leetcode.com/problems/ipo/) - *Hard*

### Backtracking

- [ ] **123**. [Sum of All Subsets XOR Total](https://leetcode.com/problems/sum-of-all-subset-xor-totals) - *Easy*
- [ ] **124**. [Subsets](https://leetcode.com/problems/subsets/) - *Medium*
- [ ] **125**. [Combination Sum](https://leetcode.com/problems/combination-sum/) - *Medium*
- [ ] **126**. [Combination Sum II](https://leetcode.com/problems/combination-sum-ii/) - *Medium*
- [ ] **127**. [Combinations](https://leetcode.com/problems/combinations/) - *Medium*
- [ ] **128**. [Permutations](https://leetcode.com/problems/permutations/) - *Medium*
- [ ] **129**. [Subsets II](https://leetcode.com/problems/subsets-ii/) - *Medium*
- [ ] **130**. [Permutations II](https://leetcode.com/problems/permutations-ii/) - *Medium*
- [ ] **131**. [Word Search](https://leetcode.com/problems/word-search/) - *Medium*
- [ ] **132**. [Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/) - *Medium*
- [ ] **133**. [Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/) - *Medium*
- [ ] **134**. [Matchsticks to Square](https://leetcode.com/problems/matchsticks-to-square/) - *Medium*
- [ ] **135**. [Partition to K Equal Sum Subsets](https://leetcode.com/problems/partition-to-k-equal-sum-subsets/) - *Medium*
- [ ] **136**. [N Queens](https://leetcode.com/problems/n-queens/) - *Hard*
- [ ] **137**. [N Queens II](https://leetcode.com/problems/n-queens-ii/) - *Hard*
- [ ] **138**. [Word Break II](https://leetcode.com/problems/word-break-ii) - *Hard*

### Tries

- [ ] **139**. [Implement Trie Prefix Tree](https://leetcode.com/problems/implement-trie-prefix-tree/) - *Medium*
- [ ] **140**. [Design Add And Search Words Data Structure](https://leetcode.com/problems/design-add-and-search-words-data-structure/) - *Medium*
- [ ] **141**. [Extra Characters in a String](https://leetcode.com/problems/extra-characters-in-a-string/) - *Medium*
- [ ] **142**. [Word Search II](https://leetcode.com/problems/word-search-ii/) - *Hard*

### Graphs

- [ ] **143**. [Island Perimeter](https://leetcode.com/problems/island-perimeter/) - *Easy*
- [ ] **144**. [Verifying An Alien Dictionary](https://leetcode.com/problems/verifying-an-alien-dictionary/) - *Easy*
- [ ] **145**. [Find the Town Judge](https://leetcode.com/problems/find-the-town-judge) - *Easy*
- [ ] **146**. [Number of Islands](https://leetcode.com/problems/number-of-islands/) - *Medium*
- [ ] **147**. [Max Area of Island](https://leetcode.com/problems/max-area-of-island/) - *Medium*
- [ ] **148**. [Clone Graph](https://leetcode.com/problems/clone-graph/) - *Medium*
- [ ] **149**. [Walls And Gates](https://leetcode.com/problems/walls-and-gates/) - *Medium*
- [ ] **150**. [Rotting Oranges](https://leetcode.com/problems/rotting-oranges/) - *Medium*
- [ ] **151**. [Pacific Atlantic Water Flow](https://leetcode.com/problems/pacific-atlantic-water-flow/) - *Medium*
- [ ] **152**. [Surrounded Regions](https://leetcode.com/problems/surrounded-regions/) - *Medium*
- [ ] **153**. [Open The Lock](https://leetcode.com/problems/open-the-lock/) - *Medium*
- [ ] **154**. [Course Schedule](https://leetcode.com/problems/course-schedule/) - *Medium*
- [ ] **155**. [Course Schedule II](https://leetcode.com/problems/course-schedule-ii/) - *Medium*
- [ ] **156**. [Graph Valid Tree](https://leetcode.com/problems/graph-valid-tree/) - *Medium*
- [ ] **157**. [Course Schedule IV](https://leetcode.com/problems/course-schedule-iv/) - *Medium*
- [ ] **158**. [Number of Connected Components In An Undirected Graph](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/) - *Medium*
- [ ] **159**. [Redundant Connection](https://leetcode.com/problems/redundant-connection/) - *Medium*
- [ ] **160**. [Accounts Merge](https://leetcode.com/problems/accounts-merge/) - *Medium*
- [ ] **161**. [Evaluate Division](https://leetcode.com/problems/evaluate-division/) - *Medium*
- [ ] **162**. [Minimum Height Trees](https://leetcode.com/problems/minimum-height-trees) - *Medium*
- [ ] **163**. [Word Ladder](https://leetcode.com/problems/word-ladder/) - *Hard*

### Advanced Graphs

- [ ] **164**. [Path with Minimum Effort](https://leetcode.com/problems/path-with-minimum-effort/) - *Medium*
- [ ] **165**. [Network Delay Time](https://leetcode.com/problems/network-delay-time/) - *Medium*
- [ ] **166**. [Reconstruct Itinerary](https://leetcode.com/problems/reconstruct-itinerary/) - *Hard*
- [ ] **167**. [Min Cost to Connect All Points](https://leetcode.com/problems/min-cost-to-connect-all-points/) - *Medium*
- [ ] **168**. [Swim In Rising Water](https://leetcode.com/problems/swim-in-rising-water/) - *Hard*
- [ ] **169**. [Alien Dictionary](https://leetcode.com/problems/alien-dictionary/) - *Hard*
- [ ] **170**. [Cheapest Flights Within K Stops](https://leetcode.com/problems/cheapest-flights-within-k-stops/) - *Medium*
- [ ] **171**. [Find Critical and Pseudo Critical Edges in Minimum Spanning Tree](https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/) - *Hard*
- [ ] **172**. [Build a Matrix With Conditions](https://leetcode.com/problems/build-a-matrix-with-conditions) - *Hard*
- [ ] **173**. [Greatest Common Divisor Traversal](https://leetcode.com/problems/greatest-common-divisor-traversal) - *Hard*

### Dp 1D

- [ ] **174**. [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/) - *Easy*
- [ ] **175**. [Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/) - *Easy*
- [ ] **176**. [N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/) - *Easy*
- [ ] **177**. [House Robber](https://leetcode.com/problems/house-robber/) - *Medium*
- [ ] **178**. [House Robber II](https://leetcode.com/problems/house-robber-ii/) - *Medium*
- [ ] **179**. [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/) - *Medium*
- [ ] **180**. [Palindromic Substrings](https://leetcode.com/problems/palindromic-substrings/) - *Medium*
- [ ] **181**. [Decode Ways](https://leetcode.com/problems/decode-ways/) - *Medium*
- [ ] **182**. [Coin Change](https://leetcode.com/problems/coin-change/) - *Medium*
- [ ] **183**. [Maximum Product Subarray](https://leetcode.com/problems/maximum-product-subarray/) - *Medium*
- [ ] **184**. [Word Break](https://leetcode.com/problems/word-break/) - *Medium*
- [ ] **185**. [Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/) - *Medium*
- [ ] **186**. [Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/) - *Medium*
- [ ] **187**. [Combination Sum IV](https://leetcode.com/problems/combination-sum-iv/) - *Medium*
- [ ] **188**. [Perfect Squares](https://leetcode.com/problems/perfect-squares/) - *Medium*
- [ ] **189**. [Integer Break](https://leetcode.com/problems/integer-break/) - *Medium*
- [ ] **190**. [Stone Game III](https://leetcode.com/problems/stone-game-iii/) - *Hard*

### Dp 2D

- [ ] **191**. [Unique Paths](https://leetcode.com/problems/unique-paths/) - *Medium*
- [ ] **192**. [Unique Paths II](https://leetcode.com/problems/unique-paths-ii/) - *Medium*
- [ ] **193**. [Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/) - *Medium*
- [ ] **194**. [Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/) - *Medium*
- [ ] **195**. [Last Stone Weight II](https://leetcode.com/problems/last-stone-weight-ii/) - *Medium*
- [ ] **196**. [Best Time to Buy And Sell Stock With Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/) - *Medium*
- [ ] **197**. [Coin Change II](https://leetcode.com/problems/coin-change-ii/) - *Medium*
- [ ] **198**. [Target Sum](https://leetcode.com/problems/target-sum/) - *Medium*
- [ ] **199**. [Interleaving String](https://leetcode.com/problems/interleaving-string/) - *Medium*
- [ ] **200**. [Stone Game](https://leetcode.com/problems/stone-game/) - *Medium*
- [ ] **201**. [Stone Game II](https://leetcode.com/problems/stone-game-ii/) - *Medium*
- [ ] **202**. [Longest Increasing Path In a Matrix](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/) - *Hard*
- [ ] **203**. [Distinct Subsequences](https://leetcode.com/problems/distinct-subsequences/) - *Hard*
- [ ] **204**. [Edit Distance](https://leetcode.com/problems/edit-distance/) - *Medium*
- [ ] **205**. [Burst Balloons](https://leetcode.com/problems/burst-balloons/) - *Hard*
- [ ] **206**. [Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/) - *Hard*

### Greedy

- [ ] **207**. [Lemonade Change](https://leetcode.com/problems/lemonade-change/) - *Easy*
- [ ] **208**. [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/) - *Medium*
- [ ] **209**. [Maximum Sum Circular Subarray](https://leetcode.com/problems/maximum-sum-circular-subarray/) - *Medium*
- [ ] **210**. [Longest Turbulent Subarray](https://leetcode.com/problems/longest-turbulent-subarray/) - *Medium*
- [ ] **211**. [Jump Game](https://leetcode.com/problems/jump-game/) - *Medium*
- [ ] **212**. [Jump Game II](https://leetcode.com/problems/jump-game-ii/) - *Medium*
- [ ] **213**. [Jump Game VII](https://leetcode.com/problems/jump-game-vii/) - *Medium*
- [ ] **214**. [Gas Station](https://leetcode.com/problems/gas-station/) - *Medium*
- [ ] **215**. [Hand of Straights](https://leetcode.com/problems/hand-of-straights/) - *Medium*
- [ ] **216**. [Dota2 Senate](https://leetcode.com/problems/dota2-senate/) - *Medium*
- [ ] **217**. [Merge Triplets to Form Target Triplet](https://leetcode.com/problems/merge-triplets-to-form-target-triplet/) - *Medium*
- [ ] **218**. [Partition Labels](https://leetcode.com/problems/partition-labels/) - *Medium*
- [ ] **219**. [Valid Parenthesis String](https://leetcode.com/problems/valid-parenthesis-string/) - *Medium*
- [ ] **220**. [Candy](https://leetcode.com/problems/candy/) - *Hard*

### Intervals

- [ ] **221**. [Insert Interval](https://leetcode.com/problems/insert-interval/) - *Medium*
- [ ] **222**. [Merge Intervals](https://leetcode.com/problems/merge-intervals/) - *Medium*
- [ ] **223**. [Non Overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals/) - *Medium*
- [ ] **224**. [Meeting Rooms](https://leetcode.com/problems/meeting-rooms/) - *Easy*
- [ ] **225**. [Meeting Rooms II](https://leetcode.com/problems/meeting-rooms-ii/) - *Medium*
- [ ] **226**. [Meeting Rooms III](https://leetcode.com/problems/meeting-rooms-iii) - *Hard*
- [ ] **227**. [Minimum Interval to Include Each Query](https://leetcode.com/problems/minimum-interval-to-include-each-query/) - *Hard*

### Math Geometry

- [ ] **228**. [Excel Sheet Column Title](https://leetcode.com/problems/excel-sheet-column-title/) - *Easy*
- [ ] **229**. [Greatest Common Divisor of Strings](https://leetcode.com/problems/greatest-common-divisor-of-strings/) - *Easy*
- [ ] **230**. [Insert Greatest Common Divisors in Linked List](https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/) - *Medium*
- [ ] **231**. [Transpose Matrix](https://leetcode.com/problems/transpose-matrix) - *Easy*
- [ ] **232**. [Rotate Image](https://leetcode.com/problems/rotate-image/) - *Medium*
- [ ] **233**. [Spiral Matrix](https://leetcode.com/problems/spiral-matrix/) - *Medium*
- [ ] **234**. [Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes/) - *Medium*
- [ ] **235**. [Happy Number](https://leetcode.com/problems/happy-number/) - *Easy*
- [ ] **236**. [Plus One](https://leetcode.com/problems/plus-one/) - *Easy*
- [ ] **237**. [Roman to Integer](https://leetcode.com/problems/roman-to-integer/) - *Easy*
- [ ] **238**. [Pow(x, n)](https://leetcode.com/problems/powx-n/) - *Medium*
- [ ] **239**. [Multiply Strings](https://leetcode.com/problems/multiply-strings/) - *Medium*
- [ ] **240**. [Detect Squares](https://leetcode.com/problems/detect-squares/) - *Medium*

### Bit Manipulation

- [ ] **241**. [Single Number](https://leetcode.com/problems/single-number/) - *Easy*
- [ ] **242**. [Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/) - *Easy*
- [ ] **243**. [Counting Bits](https://leetcode.com/problems/counting-bits/) - *Easy*
- [ ] **244**. [Add Binary](https://leetcode.com/problems/add-binary/) - *Easy*
- [ ] **245**. [Reverse Bits](https://leetcode.com/problems/reverse-bits/) - *Easy*
- [ ] **246**. [Missing Number](https://leetcode.com/problems/missing-number/) - *Easy*
- [ ] **247**. [Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/) - *Medium*
- [ ] **248**. [Reverse Integer](https://leetcode.com/problems/reverse-integer/) - *Medium*
- [ ] **249**. [Bitwise AND of Numbers Range](https://leetcode.com/problems/bitwise-and-of-numbers-range) - *Medium*
- [ ] **250**. [Minimum Array End](https://leetcode.com/problems/minimum-array-end/) - *Medium*

## Resources

- [NeetCode.io](https://neetcode.io/) - Video explanations and more
- [LeetCode](https://leetcode.com/) - Original problem source
- [The Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples

## Contributing

This is a personal practice repository. Feel free to fork and customize for your own learning!

## License

This repository is for educational purposes. All problems are sourced from LeetCode and NeetCode.
