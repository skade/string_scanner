extern crate regex;

#[derive(Debug, Clone)]
pub struct StringScanner<'input> {
    string: &'input str,
    position: usize,
    matched: Option<&'input str>
}

impl<'input> StringScanner<'input> {
    pub fn new(s: &'input str) -> StringScanner<'input> {
        StringScanner {
            string: s,
            position: 0,
            matched: None
        }
    }
    pub fn beginning_of_line(&self) -> bool {
        self.position == 0 || &self.string[self.position - 1..self.position] == "\n"
    }

    pub fn bol(&self) -> bool {
        self.beginning_of_line()
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn scan(&mut self, pattern: &str) -> Option<&'input str> {
        let regex = regex::Regex::new(pattern).unwrap();
        let res = regex.find(&self.string[self.position..]);

        if let Some(ref info) = res {
            self.position = self.position + info.end();
            self.matched = Some(info.as_str());
        }

        
        res.map(|r| r.as_str())
    }
    
    pub fn getch(&mut self) -> Option<&'input str> {
        if self.position < self.string.len() {
            let slice = &self.string[self.position..self.position+1];
            self.position += 1;
            self.matched = Some(slice);
            Some(slice)
        } else {
            self.matched = None;
            None
        }
    }

    pub fn terminate(&mut self) {
        self.position = self.string.len() - 1;
    }

    pub fn eos(&self) -> bool {
        self.position == self.string.len() - 1
    }

    pub fn matched(&self) -> Option<&'input str> {
        self.matched
    }
}

#[cfg(test)]
mod beginning_of_line {
    mod should {
        use StringScanner;

        #[test]
        fn return_true_if_the_scan_pointer_is_at_the_beginning_of_the_line_false_otherwise() {
            let mut s = StringScanner::new("This is a test");
            assert!(s.beginning_of_line());
            s.set_position(1);
            assert!(!s.beginning_of_line());

            let mut s = StringScanner::new("hello\nworld");
            assert!(s.beginning_of_line());
            s.set_position(1);
            assert!(!s.beginning_of_line());
            s.set_position(6);
            assert!(s.beginning_of_line());
        }
    }
}

#[cfg(test)]
mod terminate {
    mod should {
        use StringScanner;

        #[test]
        fn set_the_scan_pointer_to_the_end_of_the_string() {
            let mut s = StringScanner::new("This is a test");
            s.terminate();
            assert!(!s.bol());
            assert!(s.eos());
        }
    }
}

#[cfg(test)]
mod getch {
    mod should {
        use StringScanner;

        #[test]
        fn scans_one_character_and_returns_it() {
            let mut s = StringScanner::new("abc");
            assert_eq!(s.getch(), Some("a"));
            assert_eq!(s.getch(), Some("b"));
            assert_eq!(s.getch(), Some("c"));
        }

        #[test]
        fn it_returns_nil_at_the_end_of_the_string() {
            let mut s = StringScanner::new("");
            assert_eq!(s.getch(), None);

            let mut s = StringScanner::new("a");
            s.getch();
            assert_eq!(s.getch(), None);
        }
    }
}

#[cfg(test)]
mod matched {
    mod should {
        use StringScanner;

        #[test]
        fn returns_the_last_matched_string() {
            let mut s = StringScanner::new("This is a test");
            s.scan(r#"\w+"#); // TODO: use s.match port instead
            assert_eq!(s.matched(), Some("This"));
            s.getch();
            assert_eq!(s.matched(), Some(" "));
            //assert_eq!(s.scan(r#""#), Some(""));
        }

        #[test]
        fn returns_nil_if_theres_no_match() {
            let mut s = StringScanner::new("This is a test");
            s.scan(r#"\d+"#);
            assert_eq!(s.matched(), None);

        }
    }
}

#[cfg(test)]
mod scan {
    mod should {
        use StringScanner;

        #[test]
        fn return_the_matched_string() {
            let mut s = StringScanner::new("This is a test");
            assert_eq!(s.scan(r#"\w+"#), Some("This"));
            assert_eq!(s.scan(r#"..."#), Some(" is"));
            assert_eq!(s.scan(r#""#), Some(""));
            assert_eq!(s.scan(r#"\s+"#), Some(" "));
        }

        #[test]
        fn can_drop_string_scanner() {
            let string = "This is a test";
            let mut s = StringScanner::new(string);
            let found_substring = s.scan(r#"\w+"#);
            drop(s);
            assert_eq!(found_substring, Some("This"));
        }

        #[test]
        fn treats_caret_as_matching_from_the_beginning_of_the_current_position() {
            let mut s = StringScanner::new("This is a test");

            assert_eq!(s.scan(r#"\w+"#), Some("This"));
            assert_eq!(s.scan(r#"^\d"#), None);
            assert_eq!(s.scan(r#"^\s"#), Some(" "));
        }

        #[test]
        fn returns_none_if_there_is_no_match() {
            let mut s = StringScanner::new("This is a test");

            assert_eq!(s.scan(r#"\d"#), None);
        }

        #[test]
        fn returns_non_if_there_is_no_more_to_scan() {
            let mut s = StringScanner::new("This is a test");

            assert_eq!(s.scan(r#"[\w\s]+"#), Some("This is a test"));
            assert_eq!(s.scan(r#"\w+"#), None);
        }

        #[test]
        fn returns_an_empty_string_when_the_pattern_matches_empty() {
            let mut s = StringScanner::new("This is a test");

            assert_eq!(s.scan(r#".*"#), Some("This is a test"));
            assert_eq!(s.scan(r#".*"#), Some(""));
            assert_eq!(s.scan(r#"."#), None);
        }
    }
}
