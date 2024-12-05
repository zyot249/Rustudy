/*
Given:
- an integer array (hand): hand[i] is the value of ith card
- groupSize

Return:
- can rearrange cards into groups of (groupSize) of consecutive cards?

Constraints:
- arr len: [1, 10^4]
- hand[i]: [0, 10^9]
- groupSize: [1, arr len]
*/

struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() as i32 % group_size != 0 {
            return false;
        }

        let mut amount_by_card :BTreeMap<i32, i32> = BTreeMap::new();
        //Count the amount of each card
        hand.into_iter().for_each(|card| {
            *amount_by_card.entry(card).or_insert(0) += 1;
        });

        while let Some((&first_card, _)) = amount_by_card.first_key_value() {
            // Try to form a group starting with first_card
            for card in first_card..first_card + group_size {
                match amount_by_card.get_mut(&card) {
                    Some(count) => {
                        *count -= 1;
                        if *count == 0 {
                            amount_by_card.remove(&card);
                        }
                    }
                    None => return false,
                }
            }
        }
    
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_n_straight_hand() {
        let hand = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
        let group_size = 3;
        let result = Solution::is_n_straight_hand(hand, group_size);
        assert_eq!(result, true);
    }
}
