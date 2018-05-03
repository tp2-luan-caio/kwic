pub mod input_manager {
    pub trait InputManager {
        // Constructor
        fn new();

        // Get a specific line
        fn line(index: i32) -> str;

        // Get the total number of lines
        fn length() -> i32;
    }
}