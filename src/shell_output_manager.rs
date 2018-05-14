pub mod shell_output_manager {
    use output_manager::output_manager::OutputManager;
    use std::collections::HashMap;

    pub struct ShellOutputManager{ index: HashMap<String, (String, usize)> }

    impl ShellOutputManager {
        fn new(&mut self, index: HashMap<String, (String, usize)>) {
            self.index = index;
        }

        fn show(&mut self) {
            unimplemented!()
        }
    }
}