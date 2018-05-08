pub mod dblp_input_manager {

    pub struct DBLPInputManager{ pub lines: Vec<String>, }

    use curl::easy::Easy;
    use std::io::{stdout, Write};
    use rustc_serialize::json::Json;
    use input_manager::input_manager::InputManager;


    impl InputManager for DBLPInputManager {
        // Constructor
        fn new(&mut self) {
            let mut dst = Vec::new();
            let mut json;

            // Request JSON
            let mut easy = Easy::new();
            easy.url("http://dblp.org/search/publ/api?q=\"+query+\"&format=json").unwrap();
            easy.write_function(|data| {
                json = Json::from_str(data).unwrap();
                Ok(data.len())
            }).unwrap();
            easy.perform().unwrap();

        }

        // Get a specific line
        fn line(&mut self, index: usize) -> String { self.lines[index].to_string() }
        // Get the total number of lines
        fn length(&mut self) -> usize { self.lines.len() }

    }
}