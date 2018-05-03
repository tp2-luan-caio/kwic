mod input_manager;

pub mod shell_input_manager {

    pub struct ShellInputManager {lines: Vec<str>}

    impl input_manager::InputManager for ShellInputManager {
        // Constructor
        fn new() {

        }

        // Get a specific line
        fn line(index: i32) -> str {

        }

        // Get the total number of lines
        fn length() -> i32 {

        }
    }

}