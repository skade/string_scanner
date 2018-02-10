extern crate regex;

#[derive(Debug)]
pub struct StringScanner<'input> {
    string: &'input str,
    position: usize,
    matched: Option<regex::Captures<'input>>
}

trait CapturesExtension<'input> {
    fn full_match(&self) -> regex::Match<'input>;
}

impl<'input> CapturesExtension<'input> for regex::Captures<'input> {
    fn full_match(&self) -> regex::Match<'input> {
        self.get(0).unwrap()
    }
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

    pub fn check(&mut self, pattern: &str) -> Option<&'input str> {
        let regex = regex::Regex::new(pattern).unwrap();
        let matched = regex.captures(&self.string[self.position..]);

        self.matched = matched;
        self.matched.as_ref().map(|cap| cap.full_match().as_str())
    }

    pub fn check_until(&mut self, pattern: &str) -> Option<&'input str> {
        let regex = regex::Regex::new(pattern).unwrap();
        let matched = regex.captures(&self.string[self.position..]);

        self.matched = matched;
        self.matched.as_ref().map(|cap| {
            let info = cap.full_match();
            &self.string[self.position..info.end()]
        })
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
        self.matched = None;
    }

    pub fn scan(&mut self, pattern: &str) -> Option<&'input str> {
        let regex = regex::Regex::new(pattern).unwrap();
        let matched = regex.captures(&self.string[self.position..]);

        self.matched = matched;
        if let Some(ref cap) = self.matched {
            let info = cap.full_match();
            self.position += info.end();
        }

        self.matched.as_ref().map(|m| m.get(0).unwrap().as_str())
    }

    pub fn scan_until(&mut self, pattern: &str) -> Option<&'input str> {
        let regex = regex::Regex::new(pattern).unwrap();
        let matched = regex.captures(&self.string[self.position..]);

        self.matched = matched;

        if let Some(ref cap) = self.matched {
            let info = cap.full_match();
            let result = Some(&self.string[self.position..self.position+info.end()]);

            self.position += info.end();

            result
        } else {
            None
        }
    }
    
    pub fn getch(&mut self) -> Option<&'input str> {
        self.scan(".")
    }

    pub fn terminate(&mut self) {
        self.set_position(self.string.len() - 1);
    }

    pub fn eos(&self) -> bool {
        self.position == self.string.len() - 1
    }

    pub fn matched(&self) -> Option<&'input str> {
        self.matched.as_ref().map(|m| m.full_match().as_str())
    }

    pub fn pre_match(&self) -> Option<&'input str> {
        self.matched.as_ref().map(|cap| {
            let matched = cap.full_match();

            &self.string[..self.position-matched.as_str().len()]
        })
    }

    pub fn post_match(&self) -> Option<&'input str> {
        self.matched.as_ref().map(|_| {
            &self.string[self.position..]
        })
    }

    pub fn subscan(&self) -> StringScanner<'input> {
        StringScanner {
            string: &self.string[self.position..],
            position: 0,
            matched: None,
        }
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
mod check {
    mod should {
        use StringScanner;

        #[test]
        fn returns_the_value_that_scan_would_return_without_advancing_the_scan_pointer() {
            let mut s = StringScanner::new("This is a test");
            assert_eq!(s.check(r#"This"#), Some("This"));
            assert_eq!(s.matched(), Some("This"));
            assert_eq!(s.position, 0);
            assert_eq!(s.check(r#"^is"#), None);
            assert_eq!(s.matched(), None);
        }
    }
}

#[cfg(test)]
mod check_until {
    mod should {
        use StringScanner;

        #[test]
        fn returns_the_same_value_of_scan_until_but_don_t_advances_the_scan_pointer() {
            let mut s = StringScanner::new("This is a test");
            assert_eq!(s.check_until(r#"a"#), Some("This is a"));
            assert_eq!(s.position, 0);
            assert_eq!(s.matched(), Some("a"));
            assert_eq!(s.check_until(r#"test"#), Some("This is a test"));
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
            s.scan(r#"\w+"#);
            assert_eq!(s.matched(), Some("This"));
            s.getch();
            assert_eq!(s.matched(), Some(" "));
            assert_eq!(s.scan(r#""#), Some(""));
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


#[cfg(test)]
mod scan_until {
    mod should {
        use StringScanner;

        #[test]
        fn return_the_substring_up_to_and_including_the_end_of_the_match() {
            let mut s = StringScanner::new("This is a test");
            assert_eq!(s.scan_until(r#"a"#), Some("This is a"));
            assert_eq!(s.pre_match(), Some("This is "));
            assert_eq!(s.post_match(), Some(" test"));
        }

        #[test]
        fn return_none_if_theres_no_match() {
            let mut s = StringScanner::new("This is a test");

            assert_eq!(s.scan_until(r#"\d"#), None);
        }

        #[test]
        fn match_anchors_properly() {
            let mut s = StringScanner::new("This is a test");

            s.scan(r#"T"#);
            assert_eq!(s.scan_until(r#"^h"#), Some("h"));
        }
    }
}

#[cfg(test)]
mod pre_match {
    mod should {
        use StringScanner;

        #[test]
        fn return_the_pre_match_in_the_regular_expression_sense_of_the_last_scan() {
            let mut s = StringScanner::new("This is a test");
            assert_eq!(s.pre_match(), None);
            s.scan(r#"\w+\s"#);
            assert_eq!(s.pre_match(), Some(""));
            s.getch();
            assert_eq!(s.pre_match(), Some("This "));
        }

        #[test]
        fn return_nil_if_theres_no_match() {
            let mut s = StringScanner::new("This is a test");
            s.scan(r#"^\s+"#);
            assert_eq!(s.pre_match(), None);
        }

        #[test]
        fn be_more_than_just_the_data_from_the_last_match() {
            let mut s = StringScanner::new("This is a test");
            s.scan(r#"\w+"#);
            s.scan_until(r#"a te"#);
            assert_eq!(s.pre_match(), Some("This is "));
        }

        #[test]
        fn be_invalidated_when_the_scanners_position_changes() {
            let mut s = StringScanner::new("This is a test");
            s.scan_until(r#"\s+"#);
            assert_eq!(s.pre_match(), Some("This"));
            s.set_position(0);
            assert_eq!(s.pre_match(), None);
        }
    }
}

#[cfg(test)]
mod post_match {
    mod should {
        use StringScanner;

        #[test]
        fn return_the_post_match_in_the_regular_expression_sense_of_the_last_scan() {
            let mut s = StringScanner::new("This is a test");
            assert_eq!(s.post_match(), None);
            s.scan(r#"\w+\s"#);
            assert_eq!(s.post_match(), Some("is a test"));
            s.getch();
            assert_eq!(s.post_match(), Some("s a test"));
        }

        #[test]
        fn return_nil_if_theres_no_match() {
            let mut s = StringScanner::new("This is a test");
            s.scan(r#"^\s+"#);
            assert_eq!(s.post_match(), None);
        }
    }
}
