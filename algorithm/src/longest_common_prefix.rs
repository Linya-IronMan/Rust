pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("");
    }
    let mut common = String::from("");
    // TODO: 闭包
    let min_len = strs.iter().map(|str| str.len()).min().unwrap();
    let mut i = 0;

    while i < min_len {
        // TODO: unwrap 处理
        if let Some(cur_char) = strs[0].chars().nth(i) {
            let matched = strs
                .iter()
                .enumerate()
                .all(|(_index, str)| str.chars().nth(i).unwrap().eq(&cur_char));

            if !matched {
                break;
            }
            common.push(cur_char)
        } else {
            break;
        }
        i += 1;
    }

    common
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );
    }
}
