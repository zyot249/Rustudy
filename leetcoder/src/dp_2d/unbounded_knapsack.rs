/*
Problem: https://www.geeksforgeeks.org/unbounded-knapsack-repetition-items-allowed/
*/

fn solution(weight: Vec<i32>, profit: Vec<i32>, w: i32) -> i32 {
    let n = profit.len();
    let mut dp :Vec<i32> = vec![0; w as usize + 1];

    for cap in 1..w as usize + 1 {
        for i in 0..n {
            if weight[i] <= cap as i32 {
                dp[cap] = dp[cap].max(profit[i] + dp[cap - weight[i] as usize]);
            }
        }
    }

    dp[w as usize]
}

#[cfg(test)]
mod tests {
    use crate::dp_2d::unbounded_knapsack::solution;

    #[test]
    fn test_solution_1() {

        let weight = vec![1, 3, 4, 5];
        let profit = vec![10, 40, 50, 70];
        let w = 8;

        assert_eq!(solution(weight, profit, w), 110);
    }

    #[test]
    fn test_solution_2() {

        let weight = vec![2, 1];
        let profit = vec![1, 1];
        let w = 3;

        assert_eq!(solution(weight, profit, w), 3);
    }
}