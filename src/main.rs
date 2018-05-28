extern crate curl;

mod input_manager;
mod output_manager;
mod shell_input_manager;
mod text_input_manager;
mod dblp_input_manager;
mod shell_output_manager;
mod text_output_manager;
mod stop_word_manager;
mod index_manager;

fn main() {

    use input_manager::input_manager::InputManager;
    use output_manager::output_manager::OutputManager;
	use shell_input_manager::shell_input_manager::ShellInputManager;
	use text_input_manager::text_input_manager::TextInputManager;
	use dblp_input_manager::dblp_input_manager::DBLPInputManager;
    use shell_output_manager::shell_output_manager::ShellOutputManager;
    use text_output_manager::text_output_manager::TextOutputManager;
	use stop_word_manager::stop_word_manager::StopWordManager;
	use index_manager::index_manager::IndexManager;
	use std::collections::HashMap;

	// Method Input
    println!("Entrada:");
	let mut input = TextInputManager{ lines: Vec::new(), };
    input.new();

	// Stop Words
    println!("\nStop Words:");
	let mut stop = StopWordManager{ stop_words: Vec::new(), };
	stop.new();

	// Count
	let mut index = IndexManager {
		index: HashMap::new(),
		stop_words: stop,
	};
	index.count(&mut input);

	// Method Output
	println!("\nOutput!");
	let mut output = ShellOutputManager { index: index.index };
    output.show();

}