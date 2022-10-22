pub mod morokoshi {
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ArrayObject {
        pub value: Option<Box<JsonObject>>,
        pub next: Option<Box<ArrayObject>>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum JsonObject {
        Array(ArrayObject),
        String(String),
        Number(i64),
        Boolean(bool),
        Null,
    }

    pub struct MorokoshiJsonParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MorokoshiJsonParser {
        pub fn new(input: String) -> MorokoshiJsonParser {
            MorokoshiJsonParser {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn next(&mut self) -> Option<&char> {
            self.pos += 1;
            self.curr()
        }

        fn curr(&self) -> Option<&char> {
            self.input.get(self.pos)
        }

        fn skip_whitespace(&mut self) {
            while let Some(c) = self.curr() {
                if c.is_whitespace() {
                    self.next();
                } else {
                    break;
                }
            }
        }

        fn skip_delimiter(&mut self) {
            while let Some(c) = self.curr() {
                if c.is_whitespace() || c == &',' || c == &':' {
                    self.next();
                } else {
                    break;
                }
            }
        }
        pub fn parse(&mut self) -> Option<JsonObject> {
            let result = self.parse_inside();
            if self.curr().is_some() {
                // まだ読み終わっていない場合
                None
            } else {
                result
            }
        }

        fn parse_inside(&mut self) -> Option<JsonObject> {
            self.skip_whitespace();
            let a = match self.curr() {
                // Some(&'{') => self.parse_object(),
                Some(&'[') => self.parse_array(),
                Some(&'"') => self.parse_string(),
                Some(&'t') => self.parse_true(),
                Some(&'f') => self.parse_false(),
                Some(&'n') => self.parse_null(),
                Some(&'-') => self.parse_number(),
                Some(&('0'..='9')) => self.parse_number(),
                _ => None,
            };
            self.skip_whitespace();
            a
        }

        fn parse_null(&mut self) -> Option<JsonObject> {
            let result = if self.curr().unwrap() == &'n'
                && self.next().unwrap() == &'u'
                && self.next().unwrap() == &'l'
                && self.next().unwrap() == &'l'
            {
                Some(JsonObject::Null)
            } else {
                None
            };
            self.next();
            result
        }

        fn parse_false(&mut self) -> Option<JsonObject> {
            let result = if self.curr().unwrap() == &'f'
                && self.next().unwrap() == &'a'
                && self.next().unwrap() == &'l'
                && self.next().unwrap() == &'s'
                && self.next().unwrap() == &'e'
            {
                Some(JsonObject::Boolean(false))
            } else {
                None
            };
            self.next();
            result
        }

        fn parse_true(&mut self) -> Option<JsonObject> {
            let result = if self.curr().unwrap() == &'t'
                && self.next().unwrap() == &'r'
                && self.next().unwrap() == &'u'
                && self.next().unwrap() == &'e'
            {
                Some(JsonObject::Boolean(true))
            } else {
                None
            };
            self.next();
            result
        }

        fn parse_number(&mut self) -> Option<JsonObject> {
            let mut num = String::new();
            num.push(*self.curr().unwrap());

            while self.next().is_some() {
                let d = self.curr().unwrap();
                if &'0' < d && d < &'9' {
                    num.push(*d);
                } else {
                    break;
                }
            }
            num.parse().ok().map(JsonObject::Number)
        }

        fn parse_string(&mut self) -> Option<JsonObject> {
            let mut string = String::new();
            self.next();
            while self.curr().unwrap() != &'"' {
                string.push(*self.curr().unwrap());
                if self.next().is_none() {
                    return None;
                }
            }
            self.next();
            Some(JsonObject::String(string))
        }

        fn parse_array(&mut self) -> Option<JsonObject> {
            self.next(); // '['を読み飛ばす
            match self.parse_array_inside() {
                Some(array) => Some(JsonObject::Array(array)),
                None => None,
            }
        }

        fn parse_array_inside(&mut self) -> Option<ArrayObject> {
            let value = self.parse_inside();
            self.skip_delimiter();
            if self.curr().unwrap() == &']' {
                self.next();
                match value {
                    //[1] 1つの要素を持つ配列
                    Some(x) => Some(ArrayObject {
                        value: Some(Box::new(x)),
                        next: None,
                    }),
                    //[] 空の配列
                    None => Some(ArrayObject {
                        value: None,
                        next: None,
                    }),
                }
            } else {
                match self.parse_array_inside() {
                    // 2個以上の要素を持つ配列 1つめの要素はvalueに、2つめ以降はnextに入れる
                    Some(next_value) => Some(ArrayObject {
                        value: Some(Box::new(value.unwrap())),
                        next: Some(Box::new(next_value)),
                    }),
                    //多分配列ではない
                    None => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::morokoshi::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn null() {
        let a = String::from("null");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, Some(JsonObject::Null));
    }

    #[test]
    fn not_null() {
        let a = String::from("nu1l");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, None);
    }

    #[test]
    fn true_test() {
        let a = String::from("true");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, Some(JsonObject::Boolean(true)));
    }

    #[test]
    fn false_test() {
        let a = String::from("false");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, Some(JsonObject::Boolean(false)));
    }

    #[test]
    fn number_positive_test() {
        let a = String::from("123");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, Some(JsonObject::Number(123)));
    }

    #[test]
    fn number_negative_test() {
        let a = String::from("-123");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, Some(JsonObject::Number(-123)));
    }

    #[test]
    fn not_number() {
        let a = String::from("-1f23");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, None);
    }

    #[test]
    fn string() {
        let a = String::from("\"hello\"");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, Some(JsonObject::String(String::from("hello"))));
    }

    #[test]
    fn not_string() {
        let a = String::from("\"hello");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(result, None);
    }

    #[test]
    fn empty_array() {
        let a = String::from("[]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Array(ArrayObject {
                value: None,
                next: None,
            }))
        );
    }

    #[test]
    fn one_element_array() {
        let a = String::from("[123]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Array(ArrayObject {
                value:  Some(Box::new(JsonObject::Number(123))),
                next: None,
            }))
        );
    }

    #[test]
    fn one_element_array2() {
        let a = String::from("[null]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Array(ArrayObject {
                value:  Some(Box::new(JsonObject::Null)),
                next: None,
            }))
        );
    }

    #[test]
    fn one_element_array3() {
        let a = String::from("[\"Hello\"]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Array(ArrayObject {
                value:  Some(Box::new(JsonObject::String(String::from("Hello")))),
                next: None,
            }))
        );
    }

    #[test]
    fn two_element_array() {
        let a = String::from("[123,456]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Array(ArrayObject {
                value:  Some(Box::new(JsonObject::Number(123))),
                next: Some(Box::new(ArrayObject {
                    value: Some(Box::new(JsonObject::Number(456))),
                    next: None,
                })),
            }))
        );
    }
    #[test]
    fn two_element_array2() {
        let a = String::from("[\"Hello\",\"World\"]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Array(ArrayObject {
                value:  Some(Box::new(JsonObject::String(String::from("Hello")))),
                next: Some(Box::new(ArrayObject {
                    value: Some(Box::new(JsonObject::String(String::from("World")))),
                    next: None,
                })),
            }))
        );
    }

    #[test]
    fn three_element_array() {
        let a = String::from("[123,true,\"Hello\"]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Array(ArrayObject {
                value:  Some(Box::new(JsonObject::Number(123))),
                next: Some(Box::new(ArrayObject {
                    value: Some(Box::new(JsonObject::Boolean(true))),
                    next: Some(Box::new(ArrayObject {
                        value: Some(Box::new(JsonObject::String(String::from("Hello")))),
                        next: None,
                    })),
                })),
            }))
        );
    }
}
