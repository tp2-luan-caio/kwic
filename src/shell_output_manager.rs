pub mod shell_output_manager {
    use output_manager::output_manager::OutputManager;
    use std::collections::HashMap;

    pub struct ShellOutputManager{ pub index: HashMap<String, Vec<(String, i32)> > }

    impl OutputManager for ShellOutputManager {
        fn show(&mut self) {
            let mut index = &self.index;
            for vec in index {
                println!("{}:", vec.0);
                for word in vec.1 {
                    println!("{} - position: {}", word.0, word.1);
                }
                println!();
            }
        }
    }
}