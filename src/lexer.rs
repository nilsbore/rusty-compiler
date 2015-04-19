use std::str::FromStr;

pub use self::Token::{
    EOF,
    Definition,
    Extern,
    Identifier,
    Number,
    Operator,
};

#[derive(Debug)]
pub enum Token {
    EOF,
    Definition,
    Extern,
    Identifier(String),
    Number(f64),
    Operator(char),
}

// gets the next character from the input file stream
pub fn tokenize(line: &str) -> Vec<Token>
{
    //let comment_re = regex!(r"(?m)#.*\n");
    //let preprocessed = comment_re.replace_all(line, "\n");

    let mut rtn = Vec::new();

    let mut iter = line.chars();
    while let Some(lastChar) = iter.next() {
        if lastChar == ' ' {
            continue;
        }

        if lastChar.is_alphabetic() { // identifier: [a-zA-Z][a-zA-Z0-9]*
            let mut IdentifierStr: String = lastChar.to_string();
            while let Some(lastChar) = iter.next() {
                if !lastChar.is_alphabetic() {
                    break;
                }
                IdentifierStr.push(lastChar);
            }
            if IdentifierStr == "def" {
                rtn.push(Definition);
                continue;
            }
            if IdentifierStr == "extern" {
                rtn.push(Extern);
                continue;
            }
            rtn.push(Identifier(IdentifierStr));
            continue;
        }

        if lastChar.is_numeric() || lastChar == '.' {   // Number: [0-9.]+
            let mut NumStr = lastChar.to_string();
            while let Some(lastChar) = iter.next() {
                if !lastChar.is_numeric() && lastChar != '.' {
                    break;
                }
                NumStr.push(lastChar);
            }

            let NumVal = f64::from_str(&NumStr);
            match NumVal {
                Ok(value) => rtn.push(Number(value)),
                Err(_) => panic!("This should never happen..."),
            }
            continue;
        }

        if lastChar == '#' {
            // Comment until end of line.
            rtn.push(EOF);
            break;
        }

        rtn.push(Operator(lastChar));
    }

    rtn
}
