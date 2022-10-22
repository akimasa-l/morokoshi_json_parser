pub mod morokoshi {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum JsonObject {
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

        fn curr(&mut self) -> Option<&char> {
            self.input.get(self.pos)
        }

       pub fn parse(&mut self) -> Option<JsonObject> {
            while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
                self.next();
            }
            match self.curr() {
                // Some(&'{') => self.parse_object(),
                // Some(&'[') => self.parse_array(),
                // Some(&'"') => self.parse_string(),
                Some(&'t') => self.parse_true(),
                Some(&'f') => self.parse_false(),
                Some(&'n') => self.parse_null(),
                // Some(&'-') => self.parse_number(),
                // Some(&'0'...'9') => self.parse_number(),
                _ => None,
            }
        }

        fn parse_null(&mut self) -> Option<JsonObject> {
            if self.curr().unwrap() == &'n'
                && self.next().unwrap() == &'u'
                && self.next().unwrap() == &'l'
                && self.next().unwrap() == &'l'
            {
                Some(JsonObject::Null)
            } else {
                None
            }
        }

        fn parse_false(&mut self) -> Option<JsonObject> {
            if self.curr().unwrap() == &'f'
                && self.next().unwrap() == &'a'
                && self.next().unwrap() == &'l'
                && self.next().unwrap() == &'s'
                && self.next().unwrap() == &'e'
            {
                Some(JsonObject::Boolean(false))
            } else {
                None
            }
        }

        fn parse_true(&mut self) -> Option<JsonObject> {
            if self.curr().unwrap() == &'t'
                && self.next().unwrap() == &'r'
                && self.next().unwrap() == &'u'
                && self.next().unwrap() == &'e'
            {
                Some(JsonObject::Boolean(true))
            } else {
                None
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
}
