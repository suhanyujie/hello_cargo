use std::collections::HashMap;

/// highlight_string：字符串高亮处理。
/// 通过 location 定位到匹配的字符串，并使用 in_flag 和 out_flag 对其进行包裹。
pub fn highlight_string(input: &str, location: Vec<(isize, isize)>, in_flag: &str, out_flag: &str) -> String {
    let mut res_string = String::new();
    let mut found: bool = false;
    let mut end = -1;
    let mut highlight_cache: HashMap<isize, bool> = HashMap::new();
    for (l, _) in &location {
        highlight_cache.insert(*l, true);
    }
    input.chars().enumerate().for_each(|(index, c)| {
        let index = index as isize;
        let mut y = -1;
        if let Some(ok) = highlight_cache.get(&index) {
            for (pos1, pos2) in &location {
                y = *pos2;
                if index == *pos1 {
                    if !found && end <=0 {
                        res_string.push(c);
                        found = true;
                    }
                }
            }
        }
        if end > 0 && index > end {
            res_string.push_str(out_flag);
            end = 0;
        }
        y = y - 1;
        if y > end {
            end = y;
        }
    });

    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_string() {
        let loc = vec![
            (0, 4),
        ];
        assert_eq!(
            "[in]this[out]".to_string(),
            highlight_string("this", loc, "[in]", "[out]")
        );
    }
}