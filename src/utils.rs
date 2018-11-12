pub fn compare_words(s: &str, against: &str) -> usize {
	let mut against_chars = against.chars();
	for (i, c) in s.chars().enumerate() {
		let against_char = against_chars.next();
		if i >= against.len() || c != against_char.unwrap() {
			if i == 0 || c != ' ' {
				return 0;
			}
			return i;
		}
	}
	s.len()
}