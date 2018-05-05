pub mod shell_imput_manager {

    pub struct ShellInputManager{} 
    use input_manager::imput_manager::InputManager;
    
    impl InputManager for ShellInputManager {
        // Constructor
        fn new(&mut self) {
            println!("Batata");
        }
        // Get a specific line
        fn line(&mut self, index: i32) -> String {
            "Ovo é bom".to_string()
        }
        // Get the total number of lines
        fn length(&mut self) -> i32{
            0
        }        

    }

    

}