use most_profitable_path::Solution;

#[test]
fn test_most_profitable_path() {
    // Test case 1
    let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
    let bob = 3;
    let amount = vec![-2, 4, 2, -4, 6];
    assert_eq!(Solution::most_profitable_path(edges, bob, amount), 6);

    // Test case 2
    let edges = vec![vec![0, 1]];
    let bob = 1;
    let amount = vec![-7280, 2350];
    assert_eq!(Solution::most_profitable_path(edges, bob, amount), -7280);

    // Test case 3: Single node
    let edges = vec![];
    let bob = 0;
    let amount = vec![10];
    assert_eq!(Solution::most_profitable_path(edges, bob, amount), 5); // Alice gets half the profit
}