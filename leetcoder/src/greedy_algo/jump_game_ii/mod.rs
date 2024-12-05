/*
Given:
- 0-indexed array of integer (nums)
- each element in array represents max length can jump
- initially position 0

Return:
- min number of jumps to reach last position

Constraints:
- guaranteed that u can reach last position
- arr len: [1, 10^4]
- nums[i]: [0, 1000]
*/

struct MySolution;

impl MySolution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut min_jump_to_reach :Vec<i32> = Vec::new();
        min_jump_to_reach.push(0);
        for pos in 1..(nums.len() as i32) {
            let mut last_pos = pos - 1;
            let mut min_jump = -1;
            while last_pos >= 0 {
                if min_jump_to_reach[last_pos as usize] != -1 && nums[last_pos as usize] >= pos - last_pos {
                    if min_jump == -1 {
                        min_jump = min_jump_to_reach[last_pos as usize] + 1;
                    } else {
                        min_jump = min_jump.min(min_jump_to_reach[last_pos as usize] + 1);
                    }
                }

                last_pos -= 1;
            }

            min_jump_to_reach.push(min_jump);
        }

        min_jump_to_reach[nums.len() - 1]
    }
}

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let last_index = (nums.len() - 1) as usize;
        let mut curr_val = nums[0];
        if curr_val as usize >= last_index && last_index == 0 {
            return 0;
        }

        let mut res = 1;
        let mut l= 0;

        while (l + curr_val as usize) < last_index {
            let val_l = nums[l] as usize;

            match nums[l + 1..l + val_l + 1].iter().enumerate().max_by_key(|&(i, &val)| i + val as usize) {
                Some((index, &max)) => {
                    res += 1;
                    l = l + index + 1;
                    curr_val = max;
                }
                None => {
                    break;
                },
            }
        }

        res
    }
}