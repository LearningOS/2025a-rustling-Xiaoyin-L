// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.
  

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(1,1);
       
    }
    // 3. 添加一个会失败的测试来满足"让测试失败"的要求
    #[test]
    #[should_panic]  // 这个测试应该panic，否则失败
    fn failing_test() {
        assert_eq!(1, 2);  // 这个断言会失败，导致测试不通过
    }
}
