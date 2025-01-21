/*
Given:
- 2 non-negative integers num1 and num2 represented as strings

Return:
- product of num1 and num2 in string

Constraints:
- Do not convert to number
- Do not use any built-in Biginteger lib
- num1, num2 length: [1, 200]
- no leading 0 number except 0
*/

/*
result len: 6
            123
            456
            ---
            000         i = 2
             18         j = 2
            12          j = 1
            6           j = 0
            738         i = 1
            15          j = 2
           10           j = 1
           5            j = 0
*/

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let zero = '0' as u8;
        let (num1, num2) = (num1.into_bytes(), num2.into_bytes());
        let (num1_len, num2_len) = (num1.len(), num2.len());

        let mut result = vec![0 as u8; num1_len + num2_len];
        let mut carry = 0 as u8;
        for i in (0..num1_len).rev() {
            let digit1 = num1[i] - zero;
            for j in (0..num2_len).rev() {
                let digit2 = num2[j] - zero;
                let mut temp = digit1 * digit2;
                temp += result[i + j + 1] + carry;

                result[i + j + 1] = temp % 10;
                carry = temp / 10;
            }

            if carry > 0 {
                result[i] = carry;
                carry = 0;
            }
        }

        let mut start = 0;
        for i in 0..result.len() - 1 {
            if result[i] == 0 {
                start += 1;
            } else {
                break;
            }
        }

        String::from_utf8(result.drain(start..).into_iter().map(|x| x + zero).collect()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(Solution::multiply("123".to_string(), "456".to_string()), "56088");
    }
}