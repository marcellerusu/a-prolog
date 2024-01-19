#[derive(Debug)]
pub enum Token {
    Id(String),
    Int(usize),
    Str(String),
    Variable(String),
    Comma,
    Dot,
    OpenParen,
    CloseParen,
    BackArrow,
    OpenSqBrace,
    CloseSqBrace,
    Underscore,
    Eq,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
}

impl Token {
    pub fn as_id(&self) -> Option<String> {
        match self {
            Token::Id(name) => Some(name.to_owned()),
            _ => None,
        }
    }
    pub fn as_eq(&self) -> Option<()> {
        match self {
            Token::Eq => Some(()),
            _ => None,
        }
    }
    pub fn as_greater_than(&self) -> Option<()> {
        match self {
            Token::GreaterThan => Some(()),
            _ => None,
        }
    }
    pub fn as_less_than(&self) -> Option<()> {
        match self {
            Token::LessThan => Some(()),
            _ => None,
        }
    }
    pub fn as_greater_than_equal(&self) -> Option<()> {
        match self {
            Token::GreaterThanEqual => Some(()),
            _ => None,
        }
    }
    pub fn as_less_than_equal(&self) -> Option<()> {
        match self {
            Token::LessThanEqual => Some(()),
            _ => None,
        }
    }
    pub fn as_open_square_brace(&self) -> Option<()> {
        match self {
            Token::OpenSqBrace => Some(()),
            _ => None,
        }
    }
    pub fn as_close_square_brace(&self) -> Option<()> {
        match self {
            Token::CloseSqBrace => Some(()),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<usize> {
        match self {
            Token::Int(val) => Some(*val),
            _ => None,
        }
    }
    pub fn as_variable(&self) -> Option<String> {
        match self {
            Token::Variable(name) => Some(name.to_owned()),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<String> {
        match self {
            Token::Str(val) => Some(val.to_owned()),
            _ => None,
        }
    }
    pub fn as_comma(&self) -> Option<()> {
        match self {
            Token::Comma => Some(()),
            _ => None,
        }
    }
    pub fn as_open_paren(&self) -> Option<()> {
        match self {
            Token::OpenParen => Some(()),
            _ => None,
        }
    }
    pub fn as_close_paren(&self) -> Option<()> {
        match self {
            Token::CloseParen => Some(()),
            _ => None,
        }
    }
    pub fn as_dot(&self) -> Option<()> {
        match self {
            Token::Dot => Some(()),
            _ => None,
        }
    }
    pub fn as_back_arrow(&self) -> Option<()> {
        match self {
            Token::BackArrow => Some(()),
            _ => None,
        }
    }
    pub fn as_underscore(&self) -> Option<()> {
        match self {
            Token::Underscore => Some(()),
            _ => None,
        }
    }
}

pub fn tokenize(program_string: String) -> Vec<Token> {
    let mut idx = 0;
    let mut tokens: Vec<Token> = vec![];

    while idx < program_string.len() {
        if [Some(" "), Some("\n")].contains(&program_string.get(idx..=idx)) {
            idx += 1;
        } else if program_string.get(idx..=idx) == Some("(") {
            idx += 1;
            tokens.push(Token::OpenParen)
        } else if program_string.get(idx..=idx) == Some("_") {
            idx += 1;
            tokens.push(Token::Underscore)
        } else if program_string.get(idx..=idx) == Some(")") {
            idx += 1;
            tokens.push(Token::CloseParen)
        } else if program_string.get(idx..=idx) == Some("=") {
            idx += 1;
            tokens.push(Token::Eq)
        } else if program_string.get(idx..=idx) == Some(".") {
            idx += 1;
            tokens.push(Token::Dot)
        } else if program_string.get(idx..=idx) == Some(",") {
            idx += 1;
            tokens.push(Token::Comma)
        } else if program_string.get(idx..=idx + 1) == Some(":-") {
            idx += 2;
            tokens.push(Token::BackArrow)
        } else if program_string.get(idx..=idx) == Some("[") {
            idx += 1;
            tokens.push(Token::OpenSqBrace)
        } else if program_string.get(idx..=idx) == Some("]") {
            idx += 1;
            tokens.push(Token::CloseSqBrace)
        } else if program_string.get(idx..=idx + 1) == Some(">=") {
            idx += 2;
            tokens.push(Token::GreaterThanEqual)
        } else if program_string.get(idx..=idx + 1) == Some("<=") {
            idx += 2;
            tokens.push(Token::LessThanEqual)
        } else if program_string.get(idx..=idx) == Some(">") {
            idx += 1;
            tokens.push(Token::GreaterThan)
        } else if program_string.get(idx..=idx) == Some("<") {
            idx += 1;
            tokens.push(Token::LessThan)
        } else if program_string
            .get(idx..=idx)
            .filter(|char| char.chars().next().unwrap().is_numeric())
            .is_some()
        {
            let mut num: String = "".to_string();

            for char in program_string
                .chars()
                .skip(idx)
                .take_while(|char| char.is_numeric())
            {
                num += char.to_string().as_str();
            }
            idx += num.len();
            tokens.push(Token::Int(num.parse().unwrap()))
        } else if &program_string[idx..=idx] == "?" {
            let mut name: String = "".to_string();
            idx += 1;

            for char in program_string
                .chars()
                .skip(idx)
                .take_while(|c| c.is_alphanumeric())
            {
                name += char.to_string().as_str();
            }

            idx += name.len();
            tokens.push(Token::Variable(name))
        } else if &program_string[idx..=idx] == "\"" {
            let mut str: String = "".to_string();
            idx += 1;

            for char in program_string
                .chars()
                .skip(idx)
                .take_while(|char| *char != '"')
            {
                str += char.to_string().as_str();
            }
            idx += str.len() + 1;

            tokens.push(Token::Str(str))
        } else if program_string[idx..=idx]
            .chars()
            .next()
            .unwrap()
            .is_alphabetic()
        {
            let mut name: String = "".to_string();

            for char in program_string
                .chars()
                .skip(idx)
                .take_while(|c| c.is_alphanumeric() || c == &'_')
            {
                name += char.to_string().as_str();
            }

            idx += name.len();
            tokens.push(Token::Id(name))
        } else {
            panic!("no token found!")
        }
    }
    tokens
}
