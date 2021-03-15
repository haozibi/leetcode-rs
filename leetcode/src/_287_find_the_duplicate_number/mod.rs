#[allow(unused)]

/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 *
 * https://leetcode-cn.com/problems/find-the-duplicate-number/description/
 *
 * algorithms
 * Medium (66.44%)
 * Likes:    1127
 * Dislikes: 0
 * Total Accepted:    127.8K
 * Total Submissions: 192.3K
 * Testcase Example:  '[1,3,4,2,2]'
 *
 * 给定一个包含 n + 1 个整数的数组 nums ，其数字都在 1 到 n 之间（包括 1 和 n），可知至少存在一个重复的整数。
 *
 * 假设 nums 只有 一个重复的整数 ，找出 这个重复的数 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,3,4,2,2]
 * 输出：2
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,1,3,4,2]
 * 输出：3
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,1]
 * 输出：1
 *
 *
 * 示例 4：
 *
 *
 * 输入：nums = [1,1,2]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2
 * nums.length == n + 1
 * 1
 * nums 中 只有一个整数 出现 两次或多次 ，其余整数均只出现 一次
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 如何证明 nums 中至少存在一个重复的数字?
 * 你可以在不修改数组 nums 的情况下解决这个问题吗？
 * 你可以只用常量级 O(1) 的额外空间解决这个问题吗？
 * 你可以设计一个时间复杂度小于 O(n^2) 的解决方案吗？
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // 借助哈系表
        fn f1(nums: Vec<i32>) -> i32 {
            use std::collections::HashMap;

            let mut m = HashMap::with_capacity(nums.len());
            for i in &nums {
                match m.get(i) {
                    None => {
                        m.insert(i, 0);
                    }
                    Some(_) => {
                        return *i;
                    }
                }
            }
            0
        }

        fn f2(nums: Vec<i32>) -> i32 {
            // 借助排序

            let mut nums = nums;
            nums.sort();

            for i in (1..nums.len()) {
                if nums[i] == nums[i - 1] {
                    return nums[i];
                }
            }
            0
        }

        // f1(nums)
        f2(nums)
    }
}
// @lc code=end

#[allow(unused)]
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_duplicate() {
        assert_eq!(2, Solution::find_duplicate(vec![1, 3, 4, 2, 2]));
        assert_eq!(3, Solution::find_duplicate(vec![3, 1, 3, 4, 2]));
        assert_eq!(1, Solution::find_duplicate(vec![1, 1]));
        assert_eq!(1, Solution::find_duplicate(vec![1, 1, 2]));
    }
}
