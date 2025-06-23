use adorno_filter::Decorator;
use console::Key;
use console::Term;
use std::env;
use std::io::Write;
use std::string::String;

fn main() -> std::io::Result<()> {
    let detection_word: String = env::args().skip(1).next().unwrap_or("default".to_string());

    let mut filter = Decorator::new(&detection_word).unwrap();
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
