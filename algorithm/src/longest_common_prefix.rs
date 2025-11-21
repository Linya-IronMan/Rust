pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if (strs.is_empty()) {
        return String::from("");
    }
    let mut common = String::from("");
    let minLen = strs.iter().map(|str| str.len()).min().unwrap();
    let mut i = 0;

    while (i < minLen) {
        let cur_char = strs[0].chars().nth(0).unwrap();

        i += 1;
    }

    String::from("")
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
