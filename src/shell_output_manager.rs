pub mod shell_output_manager {
    use output_manager::output_manager::OutputManager;
    use std::collections::HashMap;

    pub struct ShellOutputManager{ pub index: HashMap<String, Vec<(String, i32)> > }

    impl OutputManager for ShellOutputManager {
        fn show(&mut self) {
            let index = &self.index;
            for vec in index {
                println!("\n{}:", vec.0);
                for word in vec.1 {
                    let mut a = String::from(&word.0[..]);
                    let split = a.split(" ");
                    let mut i = 1;
                    let mut aux = String::from("");
                    let mut text = String::from("");
                    for t in split {
                        if &t[..] == "-" {
                            i -= 1;
                        }
                        if i < word.1 {
                            aux.push_str(&t[..]);
                            aux.push_str(" ");
                        }
                        else {
                            text.push_str(&t[..]);
                            text.push_str(" ")
                        }
                        i += 1;
                    }
                    text.push_str(&aux[..]);
                    println!("{}", text);
                }
            }
        }
    }
}