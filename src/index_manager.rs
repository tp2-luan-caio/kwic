pub mod index_manager {
    use std::collections::HashMap;
    use input_manager::input_manager::InputManager;
    use stop_word_manager::stop_word_manager::StopWordManager;

    pub struct IndexManager{
        pub index: HashMap<String, Vec<(String, i32)> >,
        pub stop_words: StopWordManager,
    }

    impl IndexManager {
        fn word_count(&mut self, mut word_list: Vec<String>, text_line: String, index: i32) {
            if word_list.is_empty() {
                return
            }
            let word = word_list.remove(0).to_string();
            if !self.stop_words.is_stop_word(String::from(word.trim())) {
                if self.index.contains_key(&word) {
                    let data = self.index.get_mut(&word).unwrap();
                    data.push((String::from(&text_line[..]), index));
                }
                else {
                    let mut vec = Vec::new();
                    vec.push((String::from(&text_line[..]), index));
                    self.index.insert(String::from(word.trim()), vec);
                }
            }
            self.word_count(word_list, text_line, index+1);
        }

        pub fn count(&mut self, word_list: &mut InputManager) {
            let x: &[_] = &[',', '.', '\'', '"', '-', ':', ';', '?', '!', '(', ')'];
            for index in 0..word_list.length() {
                let line = word_list.line(index);
                let aux = String::from(line.trim());
                let words = aux.split(" ");
                let mut vec: Vec<String> = Vec::new();
                for word in words {
                    if !word.trim_matches(x).is_empty() {
                        vec.push(String::from(word.trim_matches(x)));
                    }
                }
                self.word_count(vec, line, 1);
            }
        }
    }
}