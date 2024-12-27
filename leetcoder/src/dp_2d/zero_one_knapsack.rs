/*
Problem: https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/
*/

fn recursion_solution(weight: Vec<i32>, profit: Vec<i32>, w: i32) -> i32 {
    fn backtrack(weight: &Vec<i32>, profit: &Vec<i32>, w: i32, n: usize) -> i32 {
        if n == 0 || w == 0 {
            return 0;
        }

        if weight[n - 1] > w {
            backtrack(weight, profit, w, n - 1)
        } else {
            backtrack(weight, profit, w, n - 1)
                .max(profit[n - 1] + backtrack(weight, profit, w - weight[n - 1], n - 1))
        }
    }

    backtrack(&weight, &profit, w, weight.len())
}

fn memorization_solution(weight: Vec<i32>, profit: Vec<i32>, w: i32) -> i32 {
    let n = profit.len();
    let mut memo: Vec<Vec<i32>> = vec![vec![-1; w as usize + 1]; n];

    fn backtrack(
        weight: &Vec<i32>,
        profit: &Vec<i32>,
        w: i32,
        idx: i32,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if idx < 0 {
            return 0;
        }

        if memo[idx as usize][w as usize] != -1 {
            return memo[idx as usize][w as usize];
        }

        if weight[idx as usize] > w {
            memo[idx as usize][w as usize] = backtrack(weight, profit, w, idx - 1, memo);
        } else {
            let rest = backtrack(weight, profit, w - weight[idx as usize], idx - 1, memo);
            memo[idx as usize][w as usize] = rest.max(profit[idx as usize] + rest);
        }

        memo[idx as usize][w as usize]
    }

    backtrack(&weight, &profit, w, n as i32 - 1, &mut memo)
}

fn tabulation_solution(weight: Vec<i32>, profit: Vec<i32>, w: i32) -> i32 {
    let n = profit.len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; w as usize + 1]; n + 1];

    for i in 1..n + 1 {
        for j in 1..(w as usize + 1) {
            if j >= weight[i - 1] as usize {
                dp[i][j] = dp[i - 1][j - weight[i - 1] as usize].max(dp[i - 1][j - weight[i - 1] as usize] + profit[i - 1]);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    dp[n][w as usize]
}

fn space_optimized_tabulation_solution(weight: Vec<i32>, profit: Vec<i32>, w: i32) -> i32 {
    let n = profit.len();
    let mut dp: Vec<i32> = vec![0; w as usize + 1];

    for i in 1..n + 1 {
        for j in (1..w as usize + 1).rev() {
            if j >= weight[i - 1] as usize {
                dp[j] = dp[j - weight[i - 1] as usize].max(dp[j - weight[i - 1] as usize] + profit[i - 1]);
            }
        }
    }

    dp[w as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursion_solution() {
        let profit = vec![60, 100, 120];
        let weight = vec![10, 20, 30];
        let w = 50;
        assert_eq!(recursion_solution(weight, profit, w), 220);
    }

    #[test]
    fn test_memorization_solution() {
        let profit = vec![60, 100, 120];
        let weight = vec![10, 20, 30];
        let w = 50;
        assert_eq!(memorization_solution(weight, profit, w), 220);
    }

    #[test]
    fn test_tabulation_solution() {
        let profit = vec![60, 100, 120];
        let weight = vec![10, 20, 30];
        let w = 50;
        assert_eq!(tabulation_solution(weight, profit, w), 220);
    }

    #[test]
    fn test_space_optimized_tabulation_solution() {
        let profit = vec![60, 100, 120];
        let weight = vec![10, 20, 30];
        let w = 50;
        assert_eq!(space_optimized_tabulation_solution(weight, profit, w), 220);
    }
}
