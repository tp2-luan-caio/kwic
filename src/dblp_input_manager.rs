pub mod dblp_input_manager {

    pub struct DBLPInputManager{ pub lines: Vec<String>, }

    struct Collector(Vec<u8>);

    use input_manager::input_manager::InputManager;
    use curl::easy::{Easy2, Handler, WriteError};

    impl Handler for Collector {
        fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }

    impl InputManager for DBLPInputManager {
        // Constructor
        fn new(&mut self) {
            // Request JSON
            let mut easy = Easy2::new(Collector(Vec::new()));
            easy.get(true).unwrap();
            easy.url("http://dblp.org/search/publ/api?q=\"+query+\"&format=json").unwrap();
            easy.perform().unwrap();

            assert_eq!(easy.response_code().unwrap(), 200);
            let contents = easy.get_ref();
            let response = String::from_utf8_lossy(&contents.0);

            // Read JSON
            let split = response.split("\n");
            for line in split {
                let mut text = String::from(line);
                if text.contains("\"title\":") {
                    let txt = &text[(text.find("\"title\":").unwrap()+9)..];
                    self.lines.push(String::from(&txt[..txt.find("\"").unwrap()]));
                }
            }

        }

        // Get a specific line
        fn line(&mut self, index: usize) -> String { self.lines[index].to_string() }
        // Get the total number of lines
        fn length(&mut self) -> usize { self.lines.len() }

    }
}