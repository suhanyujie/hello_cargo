use std::collections::HashMap;

/// highlight_string：字符串高亮处理。
/// 通过 location 定位到匹配的字符串，并使用 in_flag 和 out_flag 对其进行包裹。
pub fn highlight_string(
    input: &str,
    location: Vec<(isize, isize)>,
    in_flag: &str,
    out_flag: &str,
) -> String {
    let mut res_string = String::new();
    let mut found: bool = false;
    let mut end = -1;
    let mut highlight_cache: HashMap<isize, bool> = HashMap::new();
    for (l, _) in &location {
        highlight_cache.insert(*l, true);
    }
    input.chars().enumerate().for_each(|(index, c)| {
        found = false;
        let index = index as isize;
        if let Some(_) = highlight_cache.get(&index) {
            for (pos1, pos2) in &location {
                if index == *pos1 {
                    if !found && end <= 0 {
                        res_string.push_str(in_flag);
                        found = true;
                    }
                    let y = *pos2 - 1;
                    if y > end {
                        end = y;
                    }
                }
            }
        }
        if end > 0 && index > end {
            res_string.push_str(out_flag);
            end = 0;
        }
        res_string.push(c);
        if index == end && end != -1 {
            res_string.push_str(out_flag);
            end = 0;
        }
    });

    return res_string;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_string() {
        let loc = vec![(0, 4)];
        assert_eq!(
            "[in]this[out]".to_string(),
            highlight_string("this", loc, "[in]", "[out]")
        );
        assert_eq!(
            "__1____1____1__".to_string(),
            highlight_string("111", vec![(0, 1), (1, 2), (2, 3)], "__", "__")
        );
    }
}
