#[allow(unused)]

/*
 * @lc app=leetcode.cn id=189 lang=rust
 *
 * [189] 旋转数组
 *
 * https://leetcode-cn.com/problems/rotate-array/description/
 *
 * algorithms
 * Medium (45.76%)
 * Likes:    916
 * Dislikes: 0
 * Total Accepted:    236.2K
 * Total Submissions: 516K
 * Testcase Example:  '[1,2,3,4,5,6,7]\n3'
 *
 * 给定一个数组，将数组中的元素向右移动 k 个位置，其中 k 是非负数。
 *
 *
 *
 * 进阶：
 *
 *
 * 尽可能想出更多的解决方案，至少有三种不同的方法可以解决这个问题。
 * 你可以使用空间复杂度为 O(1) 的 原地 算法解决这个问题吗？
 *
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: nums = [1,2,3,4,5,6,7], k = 3
 * 输出: [5,6,7,1,2,3,4]
 * 解释:
 * 向右旋转 1 步: [7,1,2,3,4,5,6]
 * 向右旋转 2 步: [6,7,1,2,3,4,5]
 * 向右旋转 3 步: [5,6,7,1,2,3,4]
 *
 *
 * 示例 2:
 *
 *
 * 输入：nums = [-1,-100,3,99], k = 2
 * 输出：[3,99,-1,-100]
 * 解释:
 * 向右旋转 1 步: [99,-1,-100,3]
 * 向右旋转 2 步: [3,99,-1,-100]
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * -2^31
 * 0
 *
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // 创建 2 倍长度的 vec，然后计算偏移量
        // 不考虑性能，先跑起来
        fn f1(nums: &mut Vec<i32>, k: i32) {
            let k = k as usize % nums.len();

            let mut nv = nums.clone();
            nv.extend(nv.clone().iter());

            let mut j = 0;

            for i in ((nums.len() - k)..(nums.len() - k + nums.len())) {
                nums[j] = nv[i];
                j = j + 1;
            }
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
    fn rotate() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7];
        let want = vec![5, 6, 7, 1, 2, 3, 4];
        Solution::rotate(&mut input, 3);
        assert_eq!(want, input);
    }
}
