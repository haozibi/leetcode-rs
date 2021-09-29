#[allow(unused)]

/*
 * @lc app=leetcode.cn id=219 lang=rust
 *
 * [219] 存在重复元素 II
 *
 * https://leetcode-cn.com/problems/contains-duplicate-ii/description/
 *
 * algorithms
 * Easy (41.27%)
 * Likes:    251
 * Dislikes: 0
 * Total Accepted:    82.2K
 * Total Submissions: 199.1K
 * Testcase Example:  '[1,2,3,1]\n3'
 *
 * 给定一个整数数组和一个整数 k，判断数组中是否存在两个不同的索引 i 和 j，使得 nums [i] = nums [j]，并且 i 和 j 的差的
 * 绝对值 至多为 k。
 *
 *
 *
 * 示例 1:
 *
 * 输入: nums = [1,2,3,1], k = 3
 * 输出: true
 *
 * 示例 2:
 *
 * 输入: nums = [1,0,1,1], k = 1
 * 输出: true
 *
 * 示例 3:
 *
 * 输入: nums = [1,2,3,1,2,3], k = 2
 * 输出: false
 *
 */

// @lc code=start
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        // 哈系表嗦哈
        fn f1(nums: Vec<i32>, k: i32) -> bool {
            use std::collections::HashMap;

            let mut m = HashMap::with_capacity(nums.len());
            for (index, v) in nums.iter().enumerate() {
                match m.get_mut(v) {
                    None => {
                        m.insert(v, vec![index]);
                    }
                    Some(e) => {
                        e.push(index);
                    }
                }
            }

            for (key, value) in m.iter() {
                if value.len() >= 2 {
                    for i in (1..value.len()) {
                        if (value[i - 1] - value[i]) <= k as usize {
                            return true;
                        }
                    }
                }
            }

            false
        }

        f1(nums, k)
    }
}
// @lc code=end

#[allow(unused)]
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn contains_nearby_duplicate() {
        assert_eq!(
            true,
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
        );
        assert_eq!(
            true,
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1)
        );
        assert_eq!(
            false,
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2)
        );
        assert_eq!(2 + 2, 4);
    }
}
