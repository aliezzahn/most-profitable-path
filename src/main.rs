use most_profitable_path::Solution;

fn main() {
    // Example usage
    let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
    let bob = 3;
    let amount = vec![-2, 4, 2, -4, 6];
    let result = Solution::most_profitable_path(edges, bob, amount);
    println!("Maximum profit: {}", result); // Output: 6
}