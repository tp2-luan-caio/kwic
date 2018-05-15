pub mod output_manager {
    use std::collections::HashMap;

    pub trait OutputManager {
        // Show as Word Count
        fn show(&mut self);
    }
}