pub mod text_input_manager {

    pub struct TextInputManager{ pub lines: Vec<String>, }
    use input_manager::input_manager::InputManager;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;

    impl InputManager for TextInputManager {
        // Constructor
        fn new(&mut self) {
            let mut file_name: String = String::from("file/");
            let mut input = String::new();
            let mut file;

            // Read name file
            println!("Digite o nome do arquivo:");
            match io::stdin().read_line(&mut input) {
                Ok(..) => file_name.push_str(&input[..]),
                Err(error) => println!("error: {}", error),
            }
            input.clear();

            // Read file
            file = File::open(file_name.trim()).expect("Erro ao abrir o arquivo");
            file.read_to_string(&mut input);
            let mut split = input.split("\n");
            for line in split {
                self.lines.push(String::from(line.trim()));
            }

        }

        // Get a specific line
        fn line(&mut self, index: usize) -> String { self.lines[index].to_string() }
        // Get the total number of lines
        fn length(&mut self) -> usize { self.lines.len() }

    }
}