#[cfg(test)]
mod test {
    #[test]
    fn sanitize() {
        macro_rules! assert_sanitize {
            ($x:expr, $y:expr $(,)?) => {
                assert_eq!(crate::sanitize($x), $y.to_owned());
            };
        }
        assert_sanitize!("foo", "foo");
        assert_sanitize!("foo\\n", "foo ");
        assert_sanitize!("\\nfoo", " foo");
        assert_sanitize!("\\nfoo\\n", " foo ");
        assert_sanitize!("\nfoo", " foo");
        assert_sanitize!("\\\"foo", "\"foo");
        assert_sanitize!("\\\"foo bar baz\\\"", "\"foo bar baz\"");
        assert_sanitize!("\\\"foo\nbar\nbaz\\\"", "\"foo bar baz\"");
        assert_sanitize!("\\\"foo\\nbar\\nbaz\\\"", "\"foo bar baz\"");
        assert_sanitize!(
            "\\\"foo\n \\nbar\n \\nbaz\\\"",
            "\"foo   bar   baz\"",
        );
    }
}
