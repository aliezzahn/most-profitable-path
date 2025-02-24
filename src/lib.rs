use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Calculates the most profitable path for Alice in a tree.
    ///
    /// # Arguments
    /// * `edges` - A vector of edges representing the tree.
    /// * `bob` - The starting node of Bob.
    /// * `amount` - A vector of amounts representing the profit/loss at each node.
    ///
    /// # Returns
    /// The maximum profit Alice can achieve.
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = amount.len();
        if n == 1 {
            // Single node case: Alice and Bob are at the same node
            return amount[0] / 2;
        }

        let mut adj = vec![vec![]; n];

        // Build the adjacency list
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        // Find Bob's path to the root (node 0)
        let mut bob_path = Vec::new();
        let mut parent = vec![n; n]; // Initialize with an invalid value
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        queue.push_back(bob as usize);
        visited[bob as usize] = true;

        // BFS to find Bob's path
        while let Some(u) = queue.pop_front() {
            if u == 0 {
                break;
            }
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = u;
                    queue.push_back(v);
                }
            }
        }

        // Reconstruct Bob's path
        let mut u = 0;
        while u != bob as usize {
            bob_path.push(u);
            u = parent[u];
        }
        bob_path.push(bob as usize);
        bob_path.reverse();

        // Calculate the time when Bob reaches each node
        let mut bob_time = vec![-1; n];
        for (time, &node) in bob_path.iter().enumerate() {
            bob_time[node] = time as i32;
        }

        // DFS to calculate Alice's maximum profit
        fn dfs(u: usize, parent: usize, time: i32, adj: &Vec<Vec<usize>>, amount: &Vec<i32>, bob_time: &Vec<i32>) -> i32 {
            let mut profit = 0;
            if bob_time[u] == -1 || time < bob_time[u] {
                profit += amount[u];
            } else if time == bob_time[u] {
                profit += amount[u] / 2;
            }

            let mut max_child_profit = i32::MIN;
            for &v in &adj[u] {
                if v != parent {
                    let child_profit = dfs(v, u, time + 1, adj, amount, bob_time);
                    max_child_profit = max_child_profit.max(child_profit);
                }
            }

            if max_child_profit == i32::MIN {
                profit
            } else {
                profit + max_child_profit
            }
        }

        dfs(0, 0, 0, &adj, &amount, &bob_time)
    }
}