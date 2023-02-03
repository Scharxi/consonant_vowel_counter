fn count(input: &str) -> (usize, usize) {
    let vowels: Vec<char> = vec![
        'a',
        'e',
        'i',
        'o',
        'u',
    ];
    let mut vowels_count = 0;
    let mut consonants_count = 0;

    for c in input.chars() {
        if vowels.contains(&c) {
            vowels_count += 1;
        } else {
            if c.is_ascii_digit() || c.is_ascii_punctuation() || c.is_whitespace() {
                continue;
            }
            consonants_count += 1;
        }
    }

    (vowels_count, consonants_count)
}

#[test]
fn test_counting() {
    let word = "hello world 12 hi";
    let expected = (4, 8);
    assert_eq!(count(word), expected);
}

fn main() {
    println!("{:?}", count("aeioud1"));
}
