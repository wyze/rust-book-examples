use std::io;

const PUNCTUATION: [char; 3] = ['.', ',', '?'];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose($head, compose!($($tail),+))
    };
}

pub fn run() {
    println!("Give a word/sentence and I will convert it to pig latin.");

    loop {
        println!("Enter your word/sentence:");

        let input = get_input();
        let output = input
            .split_whitespace()
            .map(convert_word)
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", output);

        println!("Would you like to try another word/sentence? (y/n)");

        let input = get_input();

        match input.chars().nth(0) {
            Some('y') => continue,
            Some(_) | None => break,
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        println!("An error occurred while reading the input: {}", error);
    }

    input
}

fn convert_word(word: &str) -> String {
    compose!(
        shuffle_letters,
        curry1(handle_capital)(word.starts_with(char::is_uppercase)),
        handle_punctuation
    )(word)
}

fn shuffle_letters(word: &str) -> String {
    if starts_with_vowel(word) {
        format!("{}hay", word)
    } else {
        let (left, right, _) = word.chars().fold(
            (String::new(), String::new(), false),
            |(left, right, vowel_found), c| match (VOWELS.contains(&c), vowel_found) {
                (_, true) => (left, concat(right, c), vowel_found),
                (true, false) => (left, concat(right, c), true),
                (false, false) => (concat(left, c), right, vowel_found),
            },
        );

        format!("{}{}ay", right, left)
    }
}

fn handle_capital(is_capital: bool, word: String) -> String {
    if is_capital {
        format!("{}{}", &word[..1].to_uppercase(), &word[1..].to_lowercase())
    } else {
        word
    }
}

fn handle_punctuation(word: String) -> String {
    if let Some(index) = word.find(is_punctuation) {
        format!(
            "{}{}{}",
            &word[..index],
            &word[index + 1..],
            &word[index..=index],
        )
    } else {
        word
    }
}

fn is_punctuation(chr: char) -> bool {
    PUNCTUATION.contains(&chr)
}

fn starts_with_vowel(word: &str) -> bool {
    VOWELS.iter().any(|&c| word.starts_with(c))
}

fn concat(word: String, chr: char) -> String {
    format!("{}{}", word, chr)
}

fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

type Curried = Box<dyn Fn(bool) -> Box<dyn Fn(String) -> String>>;

fn curry1(f: fn(bool, String) -> String) -> Curried {
    Box::new(move |x| Box::new(move |y| f(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_word_starts_with_vowel() {
        assert_eq!(convert_word("apple"), "applehay");
    }

    #[test]
    fn test_convert_word_starts_with_consonant() {
        assert_eq!(convert_word("gloves"), "ovesglay");
    }

    #[test]
    fn test_convert_word_is_a_capital() {
        assert_eq!(convert_word("Gloves"), "Ovesglay");
    }

    #[test]
    fn test_convert_word_ignores_capital_in_middle() {
        assert_eq!(convert_word("gloVes"), "oVesglay");
    }

    #[test]
    fn test_convert_word_handles_punctuation() {
        assert_eq!(convert_word("gloves."), "ovesglay.");
    }
}
