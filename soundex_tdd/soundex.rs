#[feature(macro_rules)];

macro_rules! my_assert(
	($val1: expr, $val2:expr) => (
		if !($val1 == $val2) {
			println!("assert_eq: {} to {} ", $val1, $val2);
			fail!();
		}
	)
)




fn encode(word: ~str) -> ~str {
	return zero_pad(head(word.clone()) + encoded_digits(word.clone()));
}

fn encoded_digits(word: ~str) -> ~str {
	if word.len() > 1 {
		return ~"1";
	}
	return ~"";
}

fn head(word: ~str) -> ~str {
	return word.slice(0,1).to_owned();
}

fn zero_pad(word: ~str) -> ~str {
	let zeros_needed = 4 - word.len();
	let mut zeros = ~"";
	do zeros_needed.times {
		zeros = zeros + "0";
	}
	return word+zeros;
}

#[test]
fn retains_sole_letter_of_one_letter_word() {
	my_assert!(encode(~"A"), ~"A000");
}

#[test]
fn pads_with_zeros_to_ensure_three_digits() {
	my_assert!(encode(~"I"), ~"I000");
}

#[test]
fn replaces_consonants_with_appropriate_digits() {
	my_assert!(encode(~"Ab"), ~"A100");
}