pub mod dblp_input_manager {

    pub struct DBLPInputManager{}
    use input_manager::input_manager::InputManager;

    impl InputManager for DBLPInputManager {
        // Constructor
        fn new(&mut self) {
            println!("Batata");
        }
        // Get a specific line
        fn line(&mut self, index: usize) -> String {
            "Ovo é bom".to_string()
        }
        // Get the total number of lines
        fn length(&mut self) -> usize{
            0
        }

    }
}