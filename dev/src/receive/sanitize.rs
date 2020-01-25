/* NOTE: https://api.slack.com/docs/message-formatting */
pub fn sanitize(input: &str) -> String {
    let chars: Vec<char> = input
        .replace("&amp;", "&")
        .replace("&lt;", "")
        .replace("&gt;", "")
        .replace("\\u201c", "\"")
        .replace("\\u201d", "\"")
        .replace("\\u2018", "\"")
        .replace("\\u2019", "\"")
        .chars()
        .collect();
    let n: usize = chars.len();
    let mut output: String = String::with_capacity(n);
    match chars[0] {
        '\\' => (),
        '\n' => output.push(' '),
        c => output.push(c),
    }
    for i in 0..(n - 1) {
        match (chars[i], chars[i + 1]) {
            ('\\', 'n') | (_, '\n') => output.push(' '),
            (_, '\\') | (_, '*') | (_, '_') | (_, '~') | (_, '`') => (),
            (_, c) => output.push(c),
        }
    }
    output
}
