fn encode(word: ~str) -> ~str {
	return zero_pad(word);
}

fn zero_pad(word: ~str) -> ~str {
	return word + "000";
}

#[test]
fn retains_sole_letter_of_one_letter_word() {
	let result = encode(~"A");
	assert_eq!(result, ~"A000");
}

#[test]
fn pads_with_zeros_to_ensure_three_digits(){
	let result = encode(~"I");
	assert_eq!(result, ~"I000");
}