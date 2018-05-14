pub mod output_manager {
    use std::collections::HashMap;

    pub trait OutputManager {
        // Initialize
        fn new(&mut self, index: HashMap<String, (String, usize)>);

        // Show as Word Count
        fn show(&mut self);
    }
}