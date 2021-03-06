#[feature(macro_rules)];


macro_rules! expect_equal(
	($val1: expr, $val2:expr) => (
		if !($val1 == $val2) {
			println!("Found: {} but Expected: {} ", $val1, $val2);
			fail!();
		}
	)
)


fn encode(word: ~str) -> ~str {
	return zero_pad(first_letter_to_upper(word.clone()) + tail(encoded_digits(word)));
}

fn digit_code(c: char) -> ~str {
	match c.to_ascii().to_lower().to_char() {
		'b'|'f'|'p'|'v' => ~"1",
		'c'|'g'|'j'|'k'|'q'|'s'|'x'|'z' => ~"2",
		'd'|'t' => ~"3",
		'l' => ~"4",
		'm'|'n' => ~"5",
		'r' => ~"6",
		_ => ~""
	}
}

fn encoded_digits(word: ~str) -> ~str {
	let mut encoding = ~"";
	if (word.len() > 0) {
		let code = digit_code(to_lower(word.clone()[0] as char));
		if code.len() > 0 {
			encoding = encoding + code;	
		} else {
			encoding = encoding + to_lower(word.clone()[0] as char).to_str();
		}
	}
	let mut i = 1;
	while i < word.len() {
		if is_complete(encoding.clone()) {
			break;
		}
		if digit_code(word[i] as char) != digit_code(word[i-1] as char) {
			encoding = encoding + digit_code(word[i] as char);
		}
		i += 1;
	}
	//println!("--- {}", encoding);
	return encoding;
}

fn to_lower(c: char) -> char {
	c.to_ascii().to_lower().to_char()
}

fn to_upper(c: char) -> char {
	c.to_ascii().to_upper().to_char()
}

fn first_letter_to_upper(word: ~str) -> ~str {
	if word.len() > 0 {
		to_upper(word[0] as char).to_str()
	} else {
		~""
	}
}

fn is_complete(encoding: ~str) -> bool {
	encoding.len() == 4 // TODO: devine this to MAX - 1
}

fn head(word: ~str) -> ~str {
	if word.len() > 0 {
		return word.slice(0,1).to_owned();	
	}
	return ~"";
}

fn tail(word: ~str) -> ~str {
	if (word.len() > 1) {
		return word.slice_from(1).to_owned();
	}
	return ~"";
}

fn last_digit(encoding: ~str) -> ~str {
	if encoding.len() > 0 {
		return encoding.slice_from(encoding.len()-1).to_owned();
	}
	return ~"";
}

fn zero_pad(word: ~str) -> ~str {
	let zeros_needed = if word.len() <= 4 { 4 - word.len() } else { 0 } ;
	return word + "0".repeat(zeros_needed);
}

#[test]
fn retains_sole_letter_of_one_letter_word() {
	expect_equal!(encode(~"A"), ~"A000");
}

#[test]
fn pads_with_zeros_to_ensure_three_digits() {
	expect_equal!(encode(~"I"), ~"I000");
}

#[test]
fn replaces_consonants_with_appropriate_digits() {
	expect_equal!(encode(~"Ab"), ~"A100");
	expect_equal!(encode(~"Ac"), ~"A200");
	expect_equal!(encode(~"Ax"), ~"A200");
}

#[test]
fn ignores_non_aphabetics() {
	expect_equal!(encode(~"A#"), ~"A000");
}

#[test]
fn encodes_empty_string() {
	// Undefined result. Just check it doesn't crash
	encode(~"");
}

#[test]
fn replaces_multiple_consonants_with_digits(){
	expect_equal!(encode(~"Acdl"), ~"A234");
}

#[test]
fn limit_lenght_to_4_characters() {
	expect_equal!(encode(~"Dcdlb").len(), 4);
}

#[test]
fn ignore_vowels() {
	expect_equal!(encode(~"Baeiouhycdl"), ~"B234");
}

#[test]
fn combine_duplicate_encodings() {
	expect_equal!(digit_code('b'), digit_code('f'));
	expect_equal!(digit_code('c'), digit_code('g'));
	expect_equal!(digit_code('d'), digit_code('t'));

	expect_equal!(encode(~"Abfcgdt"), ~"A123");
}

#[test]
fn uppercases_firs_letter() {
	expect_equal!(encode(~"abcd")[0] as char, 'A');
}

#[test]
fn ignores_vowel_like_letters() {
	expect_equal!(encode(~"BaAeEiIoOuUhHyYcdl"), ~"B234");
}

#[test]
fn ignore_case_for_consonants() {
	expect_equal!(encode(~"BCDL"),encode(~"Bcdl"));
}

#[test]
fn combine_duplicate_code_when_second_is_duplicate_of_first() {
	expect_equal!(encode(~"Bbcd"), ~"B230");
}

#[test]
fn does_not_combine_dup_encodings_separated_by_vowels() {
	expect_equal!(encode(~"Jbob"), ~"J110");
}
