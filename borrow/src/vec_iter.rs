


#[cfg(test)]
mod vec_iter_tests {
    // use super::*;

    /// # for item in &v1 遍历
    /// 遍历 vec 时，使用 &item 可以避免移动
    /// 此处如果使用 v1 而非 &v1
    /// 那么在循环中，v1 会被移动，导致循环结束后，v1 不再用
    #[test]
    fn for_in_vec() {
        let v1 = vec![1, 2, 3];
        for item in &v1 {
            println!("for in &v1 item: {}", item)
        }
    }


    /// # for item in v1 遍历
    /// 如果解开注释，v1 的访问会报错，因为for循环中没有使用 &v1 进行遍历
    /// v1 会被移动
    #[test]
    fn for_in_vec_after_move() {
        let v1 = vec![1, 2, 3];
        for item in v1 {
            println!("for in &v1 item: {}", item);
        }

        // 报错
        // 因为 v1 已经被移动了，所以不能再使用
        // println!("------------------ {}", v1.len());
    }

    /// # for item in v1.iter() 遍历
    /// 实际上等同于 for item in &v1 遍历
    /// 因为 v1.iter() 返回的是一个迭代器，迭代器的元素是 &T
    /// 不会发生移动
    #[test]
    fn for_in_vec_iter() {
        let v1 = vec![1, 2, 3];
        for item in v1.iter() {
            println!("[for_in_vec_iter] for in &v1 item: {}", item)
        }
    }
}