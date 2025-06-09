use std::collections::VecDeque;
use std::string::String;

const BACKSPACE: &str = "\x08";
const DECORATOR: char = '\u{2665}'; // This is a heart.

pub struct Decorator {
    max_length: usize,
    character_queue: VecDeque<char>,
    target: String,
}

impl Decorator {
    /// Creates a new decorator using the specified character sequence to match against the input
    /// stream.
    pub fn new(word_to_detect: String) -> Decorator {
        let number_of_target_characters = word_to_detect.chars().count();
        Decorator {
            max_length: number_of_target_characters,
            character_queue: VecDeque::with_capacity(number_of_target_characters),
            target: word_to_detect.to_lowercase(),
        }
    }

    fn trim_queue(&mut self) {
        while self.character_queue.len() > self.max_length {
            self.character_queue.pop_front();
        }
    }

    fn decorate(input: String) -> String {
        // Erase the characters output already (remember, we haven't output the last character
        // in the input string yet).
        let mut result = String::from(BACKSPACE.repeat(input.chars().count() - 1));

        // Output the queue surrounded by decorators.
        result.push(DECORATOR);
        result.push_str(input.as_str());
        result.push(DECORATOR);

        result
    }

    /// Updates the filter with a new character.
    pub fn process(&mut self, character: char) -> String {
        self.character_queue.push_back(character);
        self.trim_queue();
        let queue_string = String::from_iter(self.character_queue.iter());
        if queue_string.to_lowercase() == self.target {
            // Reset the queue.
            self.character_queue.clear();

            Self::decorate(queue_string)
        } else {
            String::from(character)
        }
    }
}
