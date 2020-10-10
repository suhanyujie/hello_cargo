//! 字符串的公用处理函数
//! 参考 go 语言版本的 https://github.com/boyter/go-string

use std::collections::HashMap;

/// 给字符串数组去重，删掉重复的元素。
/// 待优化：返回的值中的元素的声明周期必须和原始数组中元素的声明周期保持一致，也就是说原先的数组不能被释放。
pub fn rm_string_duplicates<'a>(elements: &mut Vec<&'a str>) -> Vec<&'a str> {
    let mut ht: HashMap<&str, bool> = HashMap::new();
    let mut new_arr: Vec<&str> = vec![];
    for key in elements {
        if let Some(_) = ht.get(key) {
            continue;
        } else {
            ht.insert(key, true);
            new_arr.push(key);
        }
    }

    return new_arr;
}

/// 数组中查找指定值。
pub fn contains(arr: &Vec<&str>, needle: &str) -> bool {
    for ele in arr {
        if needle == *ele {
            return true;
        }
    }

    return false;
}

/// 字符串的驼峰排列组合。
/// 例如输入 `abc`，则输出为 `"ABC", "aBC", "AbC", "abC", "ABc", "aBc", "Abc", "abc"` 组成的数组。
pub fn permute_case<'a>(input: &'a str) -> Vec<String> {
    let l = input.len();
    // 最多的可能性
    let max = 1 << l;
    let mut conbinations: Vec<String> = Vec::with_capacity(max);
    let mut i = 0;
    // 声明好字符串数组，用于存放创建的字符串
    while i < max {
        let mut tmp_str: String = String::new();
        for (idx, val) in input.char_indices() {
            if i & (1 << idx) == 0 {
                tmp_str += &val.to_uppercase().to_string();
            } else {
                tmp_str += &val.to_lowercase().to_string();
            }
        }
        {
            conbinations.push(tmp_str);
        }
        i += 1;
    }

    return conbinations;
}

/// 检查输入的基于 utf-8 的字符串是否是空格字符。
/// 空格字符包含：`\t`, `\n`, `\v`, `\f`, `\r`, `' '`, U+0085 (NEL), U+00A0 (NBSP)。
pub fn is_space(first_byte: u8, next_byte: u8) -> bool {
    match first_byte {
        _n @ 9..=13 => return true,// \t, \n, \f, \r
        32 => return true,// SPACE
        194 => {
            if next_byte == 133 {// NEL
                return true
            } else if next_byte == 160 {// NBSP
                return true;
            }
        },
        _ => return false,
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rm_string_duplicates() {
        let mut a1 = vec!["s1", "s2", "s3", "s4", "s1"];
        let res = rm_string_duplicates(&mut a1);
        assert_eq!(vec!["s1", "s2", "s3", "s4"], res);
    }

    #[test]
    fn test_contains() {
        let a1 = vec!["s1", "s2", "s3", "s4", "s1"];
        let res = contains(&a1, "s2");
        assert_eq!(true, res);
        assert_eq!(false, contains(&a1, "hello"));
    }

    #[test]
    fn test_permute_case() {
        let res = permute_case("abc");
        // assert_eq!(vec!["abc", "acb", "bac", "cab", "cba"], res);
        assert_eq!(vec!["ABC", "aBC", "AbC", "abC", "ABc", "aBc", "Abc", "abc"], res);
    }

    #[test]
    fn test_is_space() {
        // 32 代表空格:`' '`
        assert!(is_space(32, 0));
        assert!(!is_space(33, 98));
    }
}
