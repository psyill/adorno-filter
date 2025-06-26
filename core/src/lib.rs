use std::collections::VecDeque;

const BACKSPACE: &str = "\x08";
const DECORATOR: char = '\u{2665}'; // This is a heart.

#[derive(Debug)]
pub struct Decorator {
    max_length: usize,
    character_queue: VecDeque<char>,
    target: String,
}

#[derive(Debug)]
pub struct Error(&'static str);

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Detector error: {}", self.0)
    }
}

impl core::convert::From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl std::error::Error for Error {}

impl Decorator {
    /// Creates a new decorator using the specified character sequence to match against the input
    /// stream. The sequence to detect must be non-empty.
    pub fn new(word_to_detect: &str) -> Result<Decorator> {
        let number_of_target_characters = word_to_detect.chars().count();
        if number_of_target_characters < 1 {
            Err("Can't detect empty target".into())
        } else {
            Ok(Decorator {
                max_length: number_of_target_characters,
                character_queue: VecDeque::with_capacity(number_of_target_characters),
                target: word_to_detect.to_lowercase(),
            })
        }
    }

    fn trim_queue(&mut self) {
        while self.character_queue.len() > self.max_length {
            self.character_queue.pop_front();
        }
    }

    fn decorate(input: &str) -> String {
        // Erase the characters output already (remember, we haven't output the last character
        // in the input string yet).
        let mut result = BACKSPACE.repeat(input.chars().count() - 1);

        // Output the queue surrounded by decorators.
        result.push(DECORATOR);
        result.push_str(input);
        result.push(DECORATOR);

        result
    }

    /// Updates the filter with a new character.
    /// Returns a replacement string for the character input.
    pub fn process(&mut self, character: char) -> String {
        self.character_queue.push_back(character);
        self.trim_queue();
        let queue_string = String::from_iter(self.character_queue.iter());
        if queue_string.to_lowercase() == self.target {
            // Reset the queue.
            self.character_queue.clear();

            Self::decorate(&queue_string)
        } else {
            String::from(character)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DECORATOR, Decorator};

    #[test]
    fn detect_simple_word() {
        let input = "word".to_string();
        let mut decorator = Decorator::new(&input).unwrap();
        let output = input
            .chars()
            .map(|character| decorator.process(character))
            .collect::<Vec<_>>()
            .join("");
        let expected_decorated_word: String = format!("{DECORATOR}{input}{DECORATOR}");
        assert!(output.ends_with(&expected_decorated_word));
    }

    #[test]
    fn handle_empty_detection_string() {
        let decorator = Decorator::new("");
        assert!(decorator.is_err());
    }

    #[test]
    fn detect_nothing() {
        let input = "quite some text".to_string();
        let mut decorator = Decorator::new("undetected").unwrap();
        input.chars().for_each(|ich| {
            decorator
                .process(ich)
                .chars()
                .for_each(|och| assert_ne!(och, DECORATOR))
        });
    }
}
