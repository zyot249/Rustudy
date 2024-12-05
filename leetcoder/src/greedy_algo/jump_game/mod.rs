struct Solution;
struct MySolution;

impl MySolution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut can_reachs :Vec<bool> = Vec::new();
        can_reachs.push(true);

        for pos in 1..(nums.len() as i32) {
            let mut last_pos  = pos - 1;
            let mut can_reach = false;
            while last_pos >= 0 {
                let high = pos - last_pos;
                if can_reachs[last_pos as usize] && nums[last_pos as usize] >= high {
                    can_reach = true;
                    break;
                }

                last_pos -= 1;
            }

            can_reachs.push(can_reach);
        }

        can_reachs[nums.len()-1]
    }
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut left = nums.len() - 2;
        let mut right = nums.len() - 2;
        for i in (0..nums.len() - 1).rev() {
            let jump = nums[i] as usize;
            let reach = i + jump;
            if reach > right {
                right = reach;
            } 
            if reach >= left {
                left = i;
            }
        }
        left <= 0 && right >= nums.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }
}
