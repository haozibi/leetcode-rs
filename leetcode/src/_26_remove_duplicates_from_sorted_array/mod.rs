#[allow(unused)]

/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除排序数组中的重复项
 *
 * https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/description/
 *
 * algorithms
 * Easy (52.96%)
 * Likes:    1882
 * Dislikes: 0
 * Total Accepted:    558K
 * Total Submissions: 1.1M
 * Testcase Example:  '[1,1,2]'
 *
 * 给定一个排序数组，你需要在 原地 删除重复出现的元素，使得每个元素只出现一次，返回移除后数组的新长度。
 *
 * 不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。
 *
 *
 *
 * 示例 1:
 *
 * 给定数组 nums = [1,1,2],
 *
 * 函数应该返回新的长度 2, 并且原数组 nums 的前两个元素被修改为 1, 2。
 *
 * 你不需要考虑数组中超出新长度后面的元素。
 *
 * 示例 2:
 *
 * 给定 nums = [0,0,1,1,1,2,2,3,3,4],
 *
 * 函数应该返回新的长度 5, 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4。
 *
 * 你不需要考虑数组中超出新长度后面的元素。
 *
 *
 *
 *
 * 说明:
 *
 * 为什么返回数值是整数，但输出的答案是数组呢?
 *
 * 请注意，输入数组是以「引用」方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。
 *
 * 你可以想象内部操作如下:
 *
 * // nums 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
 * int len = removeDuplicates(nums);
 *
 * // 在函数里修改输入数组对于调用者是可见的。
 * // 根据你的函数返回的长度, 它会打印出数组中该长度范围内的所有元素。
 * for (int i = 0; i < len; i++) {
 * print(nums[i]);
 * }
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut i = 1;
        for j in (1..nums.len()) {
            if nums[j] != nums[i - 1] {
                nums[i] = nums[j];
                i = i + 1;
            }
        }
        (i) as i32
    }
}
// @lc code=end

#[allow(unused)]
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn remove_duplicates() {
        // let mut v1 = vec![1, 1, 2];
        // let res = Solution::remove_duplicates(&mut v1);
        // assert_eq!(&[1, 2], v1.truncate(res as usize));

        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let res = Solution::remove_duplicates(&mut input);
        let want = [0, 1, 2, 3, 4];
        let mut got = input.clone();
        got.truncate(res as usize);
        assert_eq!(&want[..], got);
    }
}
