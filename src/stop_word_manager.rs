pub mod stop_word_manager {
    use input_manager::input_manager::InputManager;
    use shell_input_manager::shell_input_manager::ShellInputManager;
    use text_input_manager::text_input_manager::TextInputManager;
    use dblp_input_manager::dblp_input_manager::DBLPInputManager;

    pub struct StopWordManager{ pub stop_words: Vec<String>, }

    impl StopWordManager {
        // Constructor
        pub fn new(&mut self) {
            // Method Input
            let mut input = TextInputManager{ lines: Vec::new(), };
            input.new();

            let z: &[_] = &[',', '.', '\'', '"', '-', ':', ';', '?', '!', '(', ')'];
            for x in 0..input.length() {
                let line = input.line(x);
                let words = line.split(" ");
                for word in words {
                    self.stop_words.push(String::from(word.trim_matches(z)));
                }
            }
        }

        // Return true if word is a stop word
        pub fn is_stop_word(&mut self, word: String) -> bool {
            self.stop_words.contains(&word)
        }
    }
}