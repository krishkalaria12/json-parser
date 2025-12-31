use std::any::type_name;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn remove_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue, String> {
        self.remove_whitespace();

        match self.chars.peek() {
            Some(&c) if c == '{' => self.parse_object(),
            Some(&c) if c == '"' => self.parse_string(),
            Some(&c) if c == '[' => self.parse_array(),
            Some(&c) if c.is_numeric() => self.parse_number(),
            Some(&c) if c == 't' || c == 'f' => self.parse_bool(),
            Some(&c) if c == 'n' => self.parse_null(),
            Some(_) => Err("Unexpected character".to_string()),
            None => Err("EOF".to_string()),
        }
    }

    fn parse_string(&mut self) -> Result<JsonValue, String> {}

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        if self.chars.next() != Some('[') {
            return Err("Expected '['".to_string());
        }

        let mut array_items: Vec<JsonValue> = Vec::new();
        self.remove_whitespace();
        if let Some(&c) = self.chars.peek() {
            if c == ']' {
                self.chars.next(); // Eat the closing ']'
                return Ok(JsonValue::Array(array_items));
            }
        }

        loop {
            self.remove_whitespace();

            let value = self.parse()?;
            array_items.push(value);

            match self.chars.next() {
                Some(']') => break,
                Some(',') => continue,
                Some(c) => return Err(format!("Expected ',' or ']', found '{}'", c)),
                None => return Err("Unexpected End of File inside array".to_string()),
            }
        }

        Ok(JsonValue::Array(array_items))
    }

    fn parse_bool(&mut self) -> Result<JsonValue, String> {
        let mut bool_value = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphabetic() {
                bool_value.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        match bool_value.as_str() {
            "true" => Ok(JsonValue::Bool(true)),
            "false" => Ok(JsonValue::Bool(false)),
            _ => Err(format!(
                "Invalid syntax: expected 'true' or 'false', found '{}'",
                bool_value
            )),
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, String> {
        let expected = "null";

        for expected_char in expected.chars() {
            match self.chars.next() {
                Some(c) if c == expected_char => continue,
                Some(c) => return Err(format!("Expected '{}', found '{}'", expected_char, c)),
                None => return Err("Unexpected End of File while parsing null".to_string()),
            }
        }

        Ok(JsonValue::Null)
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let mut num = String::new();
        while let Some(&c) = self.chars.peek() {
            if matches!(c, '.' | '-' | 'e') || c.is_numeric() {
                num.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        match num.parse::<f64>() {
            Ok(n) => Ok(JsonValue::Number(n)),
            _ => Err(format!(
                "Invalid syntax: expected to be a number, found '{}'",
                num
            )),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {}
}

fn main() {
    let json = r#" { "key": "value", "list": [1, 2, 3] } "#;
    let mut parser = Parser::new(json);
    println!("{:?}", parser.parse());
}
