use console::Key;
use console::Term;
use std::collections::VecDeque;
use std::env;
use std::io::Write;
use std::string::String;

const BACKSPACE: &str = "\x08";
// This is a heart.
const DECORATOR: char = '\u{2665}';

struct Decorator {
    max_length: usize,
    character_queue: VecDeque<char>,
    target: String,
}

impl Decorator {
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

fn main() -> std::io::Result<()> {
    let detection_word: String = env::args().skip(1).next().unwrap_or("default".to_string());

    let mut filter = Decorator::new(detection_word);
    let mut input_stream = Term::stdout();
    loop {
        // read character from stdin
        let input_key = input_stream.read_key()?;
        let input_character: char = match input_key {
            Key::Enter => '\n',
            Key::Char(character) => character,
            Key::Escape | Key::CtrlC => break Ok(()),
            _ => continue,
        };

        // run input through filter
        let output_characters: String = filter.process(input_character);
        // print output
        input_stream.write_all(output_characters.as_bytes())?;
    }
}
