use std::collections::HashMap;

///

fn solution(cards: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for item in cards.iter() {
        if map.get(item).is_none() {
            map.insert(*item, 1);
        } else {
            let count = map.get(item).unwrap();
            map.insert(*item, count + 1);
        }
    }

    let mut max_count_number = 0;
    for (number, count) in map.iter() {
        if count.eq(&1) {
            max_count_number = *number;
        }
    }


   max_count_number
}

/// # 找单独的数
/// 在一个班级中，每位同学都拿到了一张卡片，上面有一个整数。
/// 有趣的是，除了一个数字之外，所有的数字都恰好出现了两次。
/// 现在需要你帮助班长小C快速找到那个拿了独特数字卡片的同学手上的数字是什么。
///
/// ## 要求：
///
/// - 设计一个算法，使其时间复杂度为 O(n)，其中 n 是班级的人数。
/// - 尽量减少额外空间的使用，以体现你的算法优化能力。
#[cfg(test)]
mod search_simple_number {
    use super::*;

    #[test]
    fn search_simple_number() {
        assert_eq!(solution(vec![1, 1, 2, 2, 3, 3, 4, 5, 5]), 4);
        assert_eq!(solution(vec![0, 1, 0, 1, 2]), 2);
    }
}