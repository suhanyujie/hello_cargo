#[cfg(test)]
mod tests {

    /// rust 中，将数值类的基础类型转换为字节序列，可以使用 `xxx.to_be_bytes()`
    /// 字符转换为字节序列，可以使用 `let s1 = '你'.to_string(); let bytes = s1.as_bytes();` ?
    /// 字符串转换为字节序列，可以使用 `"hello".as_bytes()`
    #[test]
    fn to_bytes() {
        // 数值转字节
        let bytes1 = (289 as i32).to_be_bytes();
        assert_eq!([0, 0, 1, 33], bytes1);
        // 字符转字节
        let s1 = '你'.to_string();
        let bytes1 = s1.as_bytes();
        assert_eq!([228, 189, 160], bytes1);
        // 字符串转字节
        let bytes1 = "hello".as_bytes();
        assert_eq!([104, 101, 108, 108, 111], bytes1);
        // ascii 码值转字节
        let val = (97 as u8).to_be_bytes();
        assert_eq!([97], val);
        // 将字节序列转为字符串
        let val = String::from_utf8_lossy(&val);
        assert_eq!("a", val);
    }

    // 以大端字节序的方式读取字节
    #[test]
    fn be_bytes() {
        let unum1: u8 = 97;
        let b1 = vec![unum1];
        let val = String::from_utf8_lossy(&b1);
        assert_eq!("a", val);
    }
}
