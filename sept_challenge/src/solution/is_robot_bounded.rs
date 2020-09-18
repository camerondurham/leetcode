struct Solution {}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut magx: i32 = 0;
        let mut magy: i32 = 0;
        let units = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        let mut i = 0;
        instructions.chars().for_each(|c| match c {
            'L' => i = (i + 3) % 4,
            'R' => i = (i + 1) % 4,
            _ => {
                magx += units[i][0];
                magy += units[i][1];
            }
        });
        (magx == 0 && magy == 0) || i > 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_robot_bounded("GGLLGG".to_string()), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_robot_bounded("GG".to_string()), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::is_robot_bounded("GL".to_string()), true);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::is_robot_bounded("GGLLGLRLL".to_string()), false);
    }

    #[test]
    fn ex5() {
        assert_eq!(Solution::is_robot_bounded("L".to_string()), true);
    }

    #[test]
    fn ex6() {
        assert_eq!(Solution::is_robot_bounded("GLRLLGLL".to_string()), true);
    }
}

/*
   Example 1:
    mag=2
    dir=2
   Input: "GGLLGG"
   Output: true
   Explanation:
   The robot moves from (0,0) to (0,2), turns 180 degrees, and then returns to (0,0).
   When repeating these instructions, the robot remains in the circle of radius 2 centered at the origin.

   Example 2:
    mag=2
    dir=0
   Input: "GG"
   Output: false
   Explanation:
   The robot moves north indefinitely.

   Example 3:
    mag=1
    dir=1
   Input: "GL"
   Output: true
   Explanation:

    GLRGLL
    mag=2
    dir=2


    GLRLLGLL

    unit vectors:
    1,0
    0,1
   -1,0
    0,-1


*/
