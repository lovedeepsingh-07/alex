use alex::{error, utils, string_vec};

#[rstest::rstest]
#[case("hello", "hello")]
#[case("hello world", "hello_world")]
#[case("hello-world", "hello_world")]
#[case("hello@world!", "hello_world_")]
#[case("123abc", "123abc")]
#[case("abc123", "abc123")]
#[case("!@#$%", "_____")]
#[case("", "")]
#[case("   ", "___")]
#[case("hello\nworld", "hello_world")]
#[case("こんにちは", "_____")]
#[case("你好 世界", "_____")]
fn sanitize_string(#[case] input: &str, #[case] expected: &str) -> Result<(), error::Error> {
    assert_eq!(utils::sanitize_string(input).as_str(), expected);
    Ok(())
}

#[rstest::rstest]
#[case("hello world", string_vec!["hello", "world"])]
#[case("hello-world", string_vec!["hello", "world"])]
#[case("hello_world", string_vec!["hello", "world"])]
#[case("hello@world!", string_vec!["hello", "world"])]
#[case("123 456", string_vec!["123", "456"])]
#[case("abc123def", string_vec!["abc123def"])]
#[case("123abc 456def", string_vec!["123abc", "456def"])]
#[case(" hello world ", string_vec!["hello", "world"])]
#[case("!hello!", string_vec!["hello"])]
#[case("hello   world", string_vec!["hello", "world"])]
#[case("hello---world", string_vec!["hello", "world"])]
#[case("hello___world", string_vec!["hello", "world"])]
#[case("!!!", string_vec![])]
#[case("   ", string_vec![])]
#[case("", string_vec![])]
#[case("hello\nworld", string_vec!["hello", "world"])]
#[case("hello\tworld", string_vec!["hello", "world"])]
#[case("こんにちは", string_vec![])]
#[case("你好 世界", string_vec![])]
#[case("hello 世界 world", string_vec!["hello", "world"])]
fn tokenize_string(#[case] input: &str, #[case] expected: Vec<String>) -> Result<(), error::Error> {
    assert_eq!(utils::tokenize_string(input), expected);
    Ok(())
}
