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
pub fn contains(arr: Vec<&str>, needle: &str) -> bool {
    for ele in arr {
        if needle == ele {
            return true;
        }
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
        let res = contains(a1, "s2");
        assert_eq!(true, res);
    }
}
