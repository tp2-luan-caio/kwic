pub mod index_manager {
    use std::collections::HashMap;
    use input_manager::input_manager::InputManager;
    use stop_word_manager::stop_word_manager::StopWordManager;

    pub struct IndexManager{
        pub index: HashMap<String, (String, i32)>,
        pub stop_words: StopWordManager,
    }

    impl IndexManager {
        pub fn count(&mut self, word_list: &InputManager) {

        }
    }
}