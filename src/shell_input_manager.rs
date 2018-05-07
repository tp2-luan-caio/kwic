pub mod shell_input_manager {

    pub struct ShellInputManager{
        pub lines: Vec<String>,
    }
    use input_manager::input_manager::InputManager;
    use std::io;

    impl InputManager for ShellInputManager {
        // Constructor
        fn new(&mut self) {
            let mut inp_lines: i32 = 0;
            let mut input = String::new();

            // Read number of lines
            println!("Insira o número de linhas: ");
            match io::stdin().read_line(&mut input) {
                Ok(..) => {
                    match input.trim().parse::<i32>() {
                        Ok(i) => inp_lines = i,
                        Err(error) => {
                            println!("error: {}", error);
                        },
                    }
                }
                Err(error) => println!("error: {}", error),
            }
            input.clear();

            // Read lines
            for x in 0..inp_lines {
                println!("Linha {}:", (x+1));
                match io::stdin().read_line(&mut input) {
                    Ok(..) => self.lines.push(String::from(input.trim())),
                    Err(error) => println!("error: {}", error),
                }
                input.clear();
            }
        }
        // Get a specific line
        fn line(&mut self, index: usize) -> String {
            self.lines[index].to_string()
        }
        // Get the total number of lines
        fn length(&mut self) -> usize {
            self.lines.len()
        }        

    }
}