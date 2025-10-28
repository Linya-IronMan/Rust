use std::collections::HashMap;
///
/// # 两数之和
/// https://leetcode.cn/problems/two-sum/
/// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
///
/// 你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。
///
/// 你可以按任意顺序返回答案。
///
///
///
/// 示例 1：
///
/// 输入：nums = [2,7,11,15], target = 9
/// 输出：[0,1]
/// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
/// 示例 2：
///
/// 输入：nums = [3,2,4], target = 6
/// 输出：[1,2]
/// 示例 3：
///
/// 输入：nums = [3,3], target = 6
/// 输出：[0,1]
///
///
/// 提示：
///
/// 2 <= nums.length <= 104
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// 只会存在一个有效答案
///

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    // enumerate 是一个枚举器，它可以同时返回元素的索引和值
    // 问题：iter 返回的迭代器 与 枚举器之间的区别是什么？
    // 回答：iter 返回的迭代器 只能返回元素的值，而枚举器可以返回元素的索引和值
    // 问题：迭代器和枚举器之间的区别
    // 回答：迭代器是一个惰性求值的序列，它可以在需要时才生成下一个元素
    // 而枚举器是一个立即求值的序列，它会在创建时就生成所有元素
    for (index, item) in nums.iter().enumerate() {
        let complete = target - item;
        if map.get(&complete).is_some() {
            // 此处为什么要使用 * 解引用？
            // 此处解引用之后，得到i32类型，而i32实现了copy trait，实际上是复制了一份，不会对map中存储的数据造成影响
            return vec![*map.get(&complete).unwrap(), index as i32];
        }
        map.insert(item, index as i32);
    }

    // 如果确保有解，此处可以直接 panic
    panic!("No solution found");
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        // 解构赋值，直接获取 j 对应的usize类型，而不是&usize
        if let Some(&j) = map.get(&complement) {
            // 返回之前找到的索引 j 和当前索引 i（转换为 i32）
            return vec![j as i32, i as i32];
        }
        // 把当前数及其索引存入 map（如果允许覆盖最后一个索引）
        map.insert(num, i);
    }

    // 如果题目保证一定有解，这里可以 panic! 或者返回空向量
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_works() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn two_sum2_works() {
        assert_eq!(two_sum2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum2(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum2(vec![3, 3], 6), vec![0, 1]);
    }
}
