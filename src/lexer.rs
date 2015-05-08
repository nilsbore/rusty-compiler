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

    let mut iter = line.chars().peekable();
    while let Some(last_char) = iter.next() {
        if last_char == ' ' {
            continue
        }

        if last_char.is_alphabetic() {
            let mut identifier_str = last_char.to_string();
            loop {
                if let Some(alphabetic_char) = iter.peek() {
                    if !alphabetic_char.is_alphabetic() {
                        break;
                    }
                }
                else {
                    break;
                }
                identifier_str.push(iter.next().unwrap());
            }
            match identifier_str.as_ref() {
                "def" => rtn.push(Definition),
                "extern" => rtn.push(Extern),
                _ => rtn.push(Identifier(identifier_str)),
            }
            continue;
        }

        if last_char.is_numeric() || last_char == '.' {   // Number: [0-9.]+
            let mut num_str = last_char.to_string();
            loop {
                if let Some(num_char) = iter.peek() {
                    if !num_char.is_numeric() && *num_char != '.' {
                        break;
                    }
                }
                else {
                    break;
                }
                num_str.push(iter.next().unwrap());
            }

            let num_val = f64::from_str(&num_str);
            match num_val {
                Ok(value) => rtn.push(Number(value)),
                Err(_) => panic!("This should never happen..."),
            }
            continue;
        }

        if last_char == '#' {
            rtn.push(EOF);
            break;
        }

        rtn.push(Operator(last_char));
    }


    /*while let Some(lastChar) = iter.next() {
        //let mut do_break = false;
        loop {
            if (iter.is_empty()) {
                break;
            }
            if lastChar == ' ' {
                break;
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
                //do_break = true;
                break; // break all instead
            }

            rtn.push(Operator(lastChar));
            break;
        }

        if lastChar == '#' {
            break; // break all instead
        }
    }*/

    rtn
}
