use crate::receive::token::Token;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub user: &'a str,
    pub text: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum Parse<'a> {
    Message(Message<'a>),
    Pong(&'a str),
}

pub fn transform<'a>(tokens: &'a [Token]) -> Option<Parse<'a>> {
    let n: usize = tokens.len();
    if (n == 0)
        || (tokens[0] != Token::OpenBracket)
        || (tokens[n - 1] != Token::CloseBracket)
    {
        return None;
    }
    let k: usize = ((n - 2) / 4) + 1;
    let mut stack: Vec<(&str, &str)> = Vec::with_capacity(k);
    let mut i: usize = 1;
    loop {
        if (i <= n)
            && ((i + 3) <= n)
            && (tokens[i + 1] == Token::Colon)
            && ((tokens[i + 3] == Token::Comma)
                || (tokens[i + 3] == Token::CloseBracket))
        {
            match (&tokens[i], &tokens[i + 2]) {
                (Token::Quotation(k), Token::Quotation(v))
                | (Token::Quotation(k), Token::Literal(v)) => {
                    stack.push((k, v));
                }
                _ => (),
            }
            i += 4;
        } else {
            break;
        }
    }
    assert!(stack.capacity() == k);
    if stack.is_empty() {
        return None;
    }
    stack.sort_by_key(|kv| kv.0);
    if (stack.len() == 11)
        && (stack[6].0 == "text")
        && (stack[8] == ("type", "message"))
        && (stack[9].0 == "user")
    {
        Some(Parse::Message(Message {
            user: stack[9].1,
            text: stack[6].1,
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
