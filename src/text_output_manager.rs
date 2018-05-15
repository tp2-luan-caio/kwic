pub mod text_output_manager {
    use output_manager::output_manager::OutputManager;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;

    pub struct TextOutputManager{ pub index: HashMap<String, Vec<(String, i32)> > }

    impl OutputManager for TextOutputManager {
        fn show(&mut self) {
            let index = &self.index;
            let mut file = File::create("file/output.txt");
            let mut text = String::from("");
            for vec in index {
                text.push_str("\n");
                text.push_str(vec.0);
                text.push_str(":\n");
                for word in vec.1 {
                    text.push_str(&word.0[..]);
                    text.push_str(" - position: ");
                    text.push_str(word.1.to_string().as_str());
                    text.push_str("\n");
                }
            }
            file.unwrap().write(text.as_ref());
        }
    }
}