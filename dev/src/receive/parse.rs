use crate::receive::token::Token;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub channel: &'a str,
    pub text: &'a str,
    pub user: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum Parse<'a> {
    Message(Message<'a>),
    Pong(&'a str),
}

pub fn transform<'a>(tokens: &'a [Token]) -> Option<Parse<'a>> {
    let n: usize = tokens.len();
    if (n == 0)
        || (tokens[0] != Token::OpenBrace)
        || (tokens[n - 1] != Token::CloseBrace)
    {
        return None;
    }
    let k: usize = ((n - 2) / 4) + 1;
    let mut stack: Vec<(&str, &str)> = Vec::with_capacity(k);
    let mut i: usize = 1;
    loop {
        if ((i + 3) <= n) && (tokens[i + 1] == Token::Colon) {
            match (&tokens[i], &tokens[i + 2]) {
                (Token::Quotation(k), Token::Quotation(v))
                | (Token::Quotation(k), Token::Literal(v)) => {
                    stack.push((k, v));
                }
                _ => (),
            }
            i += 3;
        } else if (i < n)
            && ((tokens[i] == Token::OpenBrace)
                || (tokens[i] == Token::CloseBrace)
                || (tokens[i] == Token::OpenBracket)
                || (tokens[i] == Token::CloseBracket)
                || (tokens[i] == Token::Comma))
        {
            i += 1;
        } else {
            break;
        }
    }
    debug_assert!(stack.capacity() == k);
    if stack.is_empty() {
        return None;
    }
    stack.sort_unstable_by_key(|kv| kv.0);
    if (stack.len() == 16)
        && (stack[1].0 == "channel")
        && (stack[7].0 == "text")
        && (stack[10] == ("type", "message"))
        && (stack[14].0 == "user")
    {
        Some(Parse::Message(Message {
            channel: stack[1].1,
            text: stack[7].1,
            user: stack[14].1,
        }))
    } else if (stack.len() == 11)
        && (stack[0].0 == "channel")
        && (stack[6].0 == "text")
        && (stack[8] == ("type", "message"))
        && (stack[9].0 == "user")
    {
        Some(Parse::Message(Message {
            channel: stack[0].1,
            text: stack[6].1,
            user: stack[9].1,
        }))
    } else if (stack.len() == 2)
        && (stack[0].0 == "reply_to")
        && (stack[1] == ("type", "pong"))
    {
        Some(Parse::Pong(stack[0].1))
    } else {
        None
    }
}
