use std::collections::HashMap;

pub struct TagSystem<'a> {
	rules: HashMap<&'a str, &'a str>,
	previous_word: String,
}

impl<'a> TagSystem<'a> {
	fn new(start: String) -> TagSystem<'a> {
		TagSystem {
			rules: HashMap::new(),
			previous_word: start,
		}
	}

	fn add_rule(&mut self, rule: &'a str, mapping: &'a str) {
		self.rules.insert(rule, mapping);
	}
}

impl<'a> Iterator for TagSystem<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		let tag = self.previous_word.get(0..1).unwrap();
		let append = match self.rules.get(tag) {
			Some(next) => String::from(*next).to_owned(),
			None => return None,
		};

		let rest = match self.previous_word.get(2..) {
			Some(r) => String::from(r).to_owned(),
			None => return None,
		};

		self.previous_word = rest + &append;

		Some(self.previous_word.clone())
	}
}

mod test {
	use super::*;

	#[test]
	fn given_rules_system_should_produce_next_iteration() {
		let mut system = TagSystem::new(String::from("baa"));

		/*
		a  -->  ccbaH
		 b  -->  cca
		 c  -->  cc
		*/
		system.add_rule("a", "ccbaH");
		system.add_rule("b", "cca");
		system.add_rule("c", "cc");

		assert_eq!(system.next(), Some(String::from("acca")));
		assert_eq!(system.next(), Some(String::from("caccbaH")));
		assert_eq!(system.next(), Some(String::from("ccbaHcc")));
		assert_eq!(system.next(), Some(String::from("baHcccc")));
		assert_eq!(system.next(), Some(String::from("Hcccccca")));
		assert_eq!(system.next(), None);
	}
}
