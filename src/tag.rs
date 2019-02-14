use std::collections::HashMap;

pub struct TagSystem<'a> {
	n: u64,
	alphabet: Vec<&'a str>,
	rules: HashMap<&'a str, &'a str>,
}

impl<'a> TagSystem<'a> {
	fn new(n: u64, alphabet: Vec<&'a str>) -> TagSystem<'a> {
		TagSystem {
			n,
			alphabet,
			rules: HashMap::new(),
		}
	}

	fn add_rule(&mut self, rule: &'a str, mapping: &'a str) {
		self.rules.insert(rule, mapping);
	}

	fn next_word(&self, previous_word: String) -> Option<String> {}
}

mod test {
	use super::*;

	#[test]
	fn given_rules_system_should_produce_next_iteration() {}
}
