# Most Profitable Path in a Tree

This project solves the problem of finding the most profitable path for Alice in a tree, given that Bob is also moving from a specific node to the root. The solution is implemented in Rust.

## Problem Description

Given a tree represented by edges, a starting node for Bob, and an array of amounts (profits/losses) at each node, the goal is to determine the maximum profit Alice can achieve when moving from the root (node 0) to any leaf node. Bob moves from his starting node to the root, and his path affects the amounts Alice can collect.

## Solution Approach

1. **Tree Construction**: The tree is represented using an adjacency list.
2. **Bob's Path**: BFS is used to find the path from Bob's starting node to the root.
3. **Profit Calculation**: DFS is used to calculate Alice's profit, adjusting for Bob's path.

## Usage

To use the solution, call the `most_profitable_path` function with the following parameters:

- `edges`: A vector of edges representing the tree.
- `bob`: The starting node of Bob.
- `amount`: A vector of amounts representing the profit/loss at each node.

Example:

```rust
let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
let bob = 3;
let amount = vec![-2, 4, 2, -4, 6];
let result = Solution::most_profitable_path(edges, bob, amount);
println!("Maximum profit: {}", result); // Output: 6
```

# Design Decisions

## Tree Representation

The tree is represented using an adjacency list because:

- It is efficient for traversal (O(1) access to neighbors).
- It is easy to implement and works well for sparse graphs.

## Bob's Path

BFS (Breadth-First Search) is used to find Bob's path because:

- It guarantees the shortest path in an unweighted graph.
- It is simple to implement and works well for trees.

## Profit Calculation

DFS (Depth-First Search) is used to calculate Alice's profit because:

- It allows us to explore all paths from the root to the leaves.
- It is well-suited for recursive implementations.

## Time Complexity

- **BFS for Bob's Path**: O(N), where N is the number of nodes.
- **DFS for Alice's Profit**: O(N), as each node is visited once.

## Space Complexity

- **Adjacency List**: O(N).
- **BFS and DFS**: O(N) for recursion stack and queue.

## Edge Cases

- Bob starts at the root.
- The tree has only one node.
- The tree is a straight line (no branches).
