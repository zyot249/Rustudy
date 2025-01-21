/*
Given:
- a stream of point on x-y plane

Return:
- algos for:
    - Add: add new points from stream to struct, dupplicated points are allowed and should be treated as diff points
    - Count: given a point -> count # of ways to choose 2 points from struct that form with the query point an axis-aligned square with positive area

Constraints:
- x, y: [0, 1000]
- at most 3000 calls 
*/

struct DetectSquares {
    plane: Vec<Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {

    fn new() -> Self {
        DetectSquares {
            plane: vec![vec![0; 1001]; 1001]
        }
    }
    
    //O(1)
    fn add(&mut self, point: Vec<i32>) {
        self.plane[point[0] as usize][point[1] as usize] += 1;
    }
    
    //O(size^2)
    fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        
        for x in 0..1001 {
            if point[1] == x || self.plane[point[0] as usize][x as usize] == 0 {
                continue;
            }

            let e = (x - point[1]).abs();

            let y = point[0] + e;
            if y <= 1000 {
                if self.plane[y as usize][x as usize] != 0 && self.plane[y as usize][point[1] as usize] != 0 {
                    res += self.plane[y as usize][x as usize]*self.plane[point[0] as usize][x as usize]*self.plane[y as usize][point[1] as usize];
                }
            }

            let y = point[0] - e;
            if y >= 0 {
                if self.plane[y as usize][x as usize] != 0 && self.plane[y as usize][point[1] as usize] != 0 {
                    res += self.plane[y as usize][x as usize]*self.plane[point[0] as usize][x as usize]*self.plane[y as usize][point[1] as usize];
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_squares() {
        let mut obj = DetectSquares::new();
        obj.add(vec![3, 10]);
        obj.add(vec![11, 2]);
        obj.add(vec![3, 2]);
        assert_eq!(obj.count(vec![11, 10]), 1);
        obj.add(vec![11, 2]);
        assert_eq!(obj.count(vec![11, 10]), 2);
    }
}
