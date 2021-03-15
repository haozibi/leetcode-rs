#[allow(unused)]

/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 *
 * https://leetcode-cn.com/problems/first-missing-positive/description/
 *
 * algorithms
 * Hard (40.70%)
 * Likes:    1003
 * Dislikes: 0
 * Total Accepted:    116.5K
 * Total Submissions: 286.1K
 * Testcase Example:  '[1,2,0]'
 *
 * 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
 *
 *
 *
 * 进阶：你可以实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案吗？
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,0]
 * 输出：3
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,4,-1,1]
 * 输出：2
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [7,8,9,11,12]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0
 * -2^31
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        // 关键词: 最小 正整数，会出现重复数字
        fn f1(nums: Vec<i32>) -> i32 {
            let mut nv = nums.clone();
            nv.sort();
            nv.dedup(); // 去除连续相同

            let mut j = 1;
            for i in nv.iter().filter(|x| x.is_positive()) {
                if *i != j {
                    return j;
                }
                j = j + 1;
            }

            j
        }

        f1(nums)
    }
}
// @lc code=end

#[allow(unused)]
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn first_missing_positive() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
        assert_eq!(1, Solution::first_missing_positive(vec![0]));
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
        assert_eq!(3, Solution::first_missing_positive(vec![0, 2, 2, 1, 1]));

        assert_eq!(2 + 2, 4);
    }
}
