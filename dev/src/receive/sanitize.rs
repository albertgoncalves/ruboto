/* NOTE: https://api.slack.com/docs/message-formatting */
pub fn sanitize(input: &str) -> String {
    let chars: &[u8] = input.as_bytes();
    let n: usize = chars.len();
    let mut output: String = String::with_capacity(n);
    let mut i: usize = 0;
    loop {
        if n <= i {
            break;
        }
        match chars[i] as char {
            '\n' => output.push(' '),
            '\\' => {
                if ((i + 1) < n) && (chars[i + 1] == b'n') {
                    output.push(' ');
                    i += 2;
                    continue;
                }
                if ((i + 5) < n)
                    && (chars[i + 1] == b'u')
                    && (chars[i + 2] == b'2')
                    && (chars[i + 3] == b'0')
                    && (chars[i + 4] == b'1')
                    && ((chars[i + 5] == b'c')
                        || (chars[i + 5] == b'd')
                        || (chars[i + 5] == b'8')
                        || (chars[i + 5] == b'9'))
                {
                    output.push('\"');
                    i += 6;
                    continue;
                }
            }
            '&' => {
                if ((i + 3) < n)
                    && ((chars[i + 1] == b'l') || (chars[i + 1] == b'g'))
                    && (chars[i + 2] == b't')
                    && (chars[i + 3] == b';')
                {
                    i += 4;
                    continue;
                }
                if ((i + 4) < n)
                    && (chars[i + 1] == b'a')
                    && (chars[i + 2] == b'm')
                    && (chars[i + 3] == b'p')
                    && (chars[i + 4] == b';')
                {
                    output.push('&');
                    i += 5;
                    continue;
                }
            }
            '*' | '_' | '~' | '`' => (),
            c => output.push(c),
        }
        i += 1;
    }
    output
}
