#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Fn(&'a str),
    Arg(&'a str),
}

#[allow(clippy::single_match, clippy::too_many_lines)]
pub fn transform(message: &str) -> Option<Vec<Token>> {
    if message.is_empty() {
        return None;
    }
    let n: usize = message.len();
    let k: usize = (n / 2) + 1;
    let mut stack: Vec<Token> = Vec::with_capacity(k);
    let chars: &[u8] = message.as_bytes();
    macro_rules! push_non_empty {
        ($t:expr, $i:expr, $j:expr) => {
            if $i != $j {
                stack.push($t(&message[$i..$j]));
            }
        };
    }
    let mut i: usize = 0;
    loop {
        if n <= i {
            if stack.is_empty() {
                return None;
            } else {
                debug_assert!(stack.capacity() == k);
                return Some(stack);
            }
        }
        match chars[i] {
            b' ' | b'\n' => (),
            b'\"' => {
                let mut j: usize = i;
                loop {
                    j += 1;
                    if j < n {
                        match chars[j] {
                            b'\"' => {
                                push_non_empty!(Token::Arg, i + 1, j);
                                i = j;
                                break;
                            }
                            _ => (),
                        }
                    } else {
                        return None;
                    }
                }
            }
            b'\'' => {
                let mut j: usize = i;
                loop {
                    j += 1;
                    if j < n {
                        match chars[j] {
                            b'\'' => {
                                push_non_empty!(Token::Arg, i + 1, j);
                                i = j;
                                break;
                            }
                            _ => (),
                        }
                    } else {
                        return None;
                    }
                }
            }
            b'!' => {
                let mut j: usize = i;
                loop {
                    j += 1;
                    if j < n {
                        match chars[j] {
                            b' ' | b'\n' => {
                                push_non_empty!(Token::Fn, i + 1, j);
                                i = j;
                                break;
                            }
                            _ => (),
                        }
                    } else {
                        push_non_empty!(Token::Fn, i + 1, n);
                        debug_assert!(stack.capacity() == k);
                        return Some(stack);
                    }
                }
            }
            _ => {
                let mut j: usize = i;
                loop {
                    j += 1;
                    if j < n {
                        match chars[j] {
                            b' ' | b'\n' => {
                                push_non_empty!(Token::Arg, i, j);
                                i = j;
                                break;
                            }
                            _ => (),
                        }
                    } else {
                        push_non_empty!(Token::Arg, i, n);
                        debug_assert!(stack.capacity() == k);
                        return Some(stack);
                    }
                }
            }
        }
        i += 1;
    }
}
