pub mod morokoshi {

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum JsonObject {
        List(ListObject),
        Map(MapObject),
        String(String),
        Number(i64),
        Boolean(bool),
        Null,
    }

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ListObject {
        pub value: Option<Box<JsonObject>>,
        pub next: Option<Box<ListObject>>,
        // これ、なんかもっといい書き方あるんじゃないかなぁ
        // valueがNoneのときは、nextは確実にNoneになるようにしたい
    }

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MapObject {
        pub key: Option<String>,
        pub value: Option<Box<JsonObject>>,
        pub next: Option<Box<MapObject>>,
        // これ、なんかもっといい書き方あるんじゃないかなぁ
        // keyがNoneのときは、valueとnextは確実にNoneになるようにしたい
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

        fn skip_comma(&mut self) {
            while let Some(c) = self.curr() {
                if c.is_whitespace() || c == &',' {
                    self.next();
                } else {
                    break;
                }
            }
        }

        fn skip_colon(&mut self) {
            while let Some(c) = self.curr() {
                if c.is_whitespace() || c == &':' {
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
                Some(&'{') => self.parse_map(),
                Some(&'[') => self.parse_list(),
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

        fn parse_string_inside(&mut self) -> Option<String> {
            let mut string = String::new();
            self.next();
            while self.curr().unwrap() != &'"' {
                string.push(*self.curr().unwrap());
                if self.next().is_none() {
                    return None;
                }
            }
            self.next();
            Some(string)
        }

        fn parse_string(&mut self) -> Option<JsonObject> {
            self.parse_string_inside().map(JsonObject::String)
        }

        fn parse_list(&mut self) -> Option<JsonObject> {
            self.next(); // '['を読み飛ばす
            match self.parse_list_inside() {
                Some(list) => Some(JsonObject::List(list)),
                None => None,
            }
        }

        fn parse_list_inside(&mut self) -> Option<ListObject> {
            let value = self.parse_inside();
            self.skip_comma();
            if self.curr().unwrap() == &']' {
                self.next();
                match value {
                    //[1] 1つの要素を持つ配列
                    Some(x) => Some(ListObject {
                        value: Some(Box::new(x)),
                        next: None,
                    }),
                    //[] 空の配列
                    None => Some(ListObject {
                        value: None,
                        next: None,
                    }),
                }
            } else {
                match self.parse_list_inside() {
                    // 2個以上の要素を持つ配列 1つめの要素はvalueに、2つめ以降はnextに入れる
                    Some(next_value) => Some(ListObject {
                        value: Some(Box::new(value.unwrap())),
                        next: Some(Box::new(next_value)),
                    }),
                    //多分配列ではない
                    None => None,
                }
            }
        }
        fn parse_map(&mut self) -> Option<JsonObject> {
            self.next(); // '{'を読み飛ばす
            match self.parse_map_inside() {
                Some(map) => Some(JsonObject::Map(map)),
                None => None,
            }
        }
        fn parse_map_inside(&mut self) -> Option<MapObject> {
            self.skip_whitespace();
            if self.curr().is_none() || self.curr().unwrap() != &'"' {
                return None;
            }
            let key = self.parse_string_inside();
            self.skip_colon();
            let value = self.parse_inside();
            self.skip_comma();
            if self.curr().unwrap() == &'}' {
                self.next();
                match key {
                    // 1つの要素を持つオブジェクト
                    Some(key) => Some(MapObject {
                        key: Some(key),
                        value: Some(Box::new(value.unwrap())),
                        next: None,
                    }),
                    // 空のオブジェクト
                    None => Some(MapObject {
                        key: None,
                        value: None,
                        next: None,
                    }),
                }
            } else {
                match self.parse_map_inside() {
                    // 2個以上の要素を持つオブジェクト 1つめの要素はvalueに、2つめ以降はnextに入れる
                    Some(next_value) => Some(MapObject {
                        key: key,
                        value: Some(Box::new(value.unwrap())),
                        next: Some(Box::new(next_value)),
                    }),
                    //多分オブジェクトではない
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
    fn empty_list() {
        let a = String::from("[]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::List(ListObject {
                value: None,
                next: None,
            }))
        );
    }

    #[test]
    fn one_element_list() {
        let a = String::from("[123]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::List(ListObject {
                value: Some(Box::new(JsonObject::Number(123))),
                next: None,
            }))
        );
    }

    #[test]
    fn one_element_list2() {
        let a = String::from("[null]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::List(ListObject {
                value: Some(Box::new(JsonObject::Null)),
                next: None,
            }))
        );
    }

    #[test]
    fn one_element_list3() {
        let a = String::from("[\"Hello\"]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::List(ListObject {
                value: Some(Box::new(JsonObject::String(String::from("Hello")))),
                next: None,
            }))
        );
    }

    #[test]
    fn two_elements_list() {
        let a = String::from("[123,456]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::List(ListObject {
                value: Some(Box::new(JsonObject::Number(123))),
                next: Some(Box::new(ListObject {
                    value: Some(Box::new(JsonObject::Number(456))),
                    next: None,
                })),
            }))
        );
    }
    #[test]
    fn two_elements_list2() {
        let a = String::from("[\"Hello\",\"World\"]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::List(ListObject {
                value: Some(Box::new(JsonObject::String(String::from("Hello")))),
                next: Some(Box::new(ListObject {
                    value: Some(Box::new(JsonObject::String(String::from("World")))),
                    next: None,
                })),
            }))
        );
    }

    #[test]
    fn three_elements_list() {
        let a = String::from("[123,true,\"Hello\"]");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::List(ListObject {
                value: Some(Box::new(JsonObject::Number(123))),
                next: Some(Box::new(ListObject {
                    value: Some(Box::new(JsonObject::Boolean(true))),
                    next: Some(Box::new(ListObject {
                        value: Some(Box::new(JsonObject::String(String::from("Hello")))),
                        next: None,
                    })),
                })),
            }))
        );
    }

    #[test]
    fn one_element_map() {
        let a = String::from("{\"a\":123}");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Map(MapObject {
                key: Some(String::from("a")),
                value: Some(Box::new(JsonObject::Number(123))),
                next: None,
            }))
        );
    }

    #[test]
    fn two_elements_map() {
        let a = String::from("{\"a\":\"t\",\"b\":null}");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Map(MapObject {
                key: Some(String::from("a")),
                value: Some(Box::new(JsonObject::String(String::from("t")))),
                next: Some(Box::new(MapObject {
                    key: Some(String::from("b")),
                    value: Some(Box::new(JsonObject::Null)),
                    next: None,
                })),
            }))
        );
    }

    #[test]
    fn nested_map() {
        let a = String::from("{\"a\":{\"b\":123},\"c\":null,\"d\":[1,false,{\"4\":true}]}");
        let mut parser = MorokoshiJsonParser::new(a);
        let result = parser.parse();
        assert_eq!(
            result,
            Some(JsonObject::Map(MapObject {
                key: Some(String::from("a")),
                value: Some(Box::new(JsonObject::Map(MapObject {
                    key: Some(String::from("b")),
                    value: Some(Box::new(JsonObject::Number(123))),
                    next: None,
                }))),
                next: Some(Box::new(MapObject {
                    key: Some(String::from("c")),
                    value: Some(Box::new(JsonObject::Null)),
                    next: Some(Box::new(MapObject {
                        key: Some(String::from("d")),
                        value: Some(Box::new(JsonObject::List(ListObject {
                            value: Some(Box::new(JsonObject::Number(1))),
                            next: Some(Box::new(ListObject {
                                value: Some(Box::new(JsonObject::Boolean(false))),
                                next: Some(Box::new(ListObject {
                                    value: Some(Box::new(JsonObject::Map(MapObject {
                                        key: Some(String::from("4")),
                                        value: Some(Box::new(JsonObject::Boolean(true))),
                                        next: None,
                                    }))),
                                    next: None,
                                })),
                            })),
                        }))),
                        next: None,
                    })),
                })),
            }))
        )
    }
}
