extern crate curl;

mod input_manager;
mod shell_input_manager;
mod text_input_manager;
mod dblp_input_manager;
mod stop_word_manager;

fn main() {

    use input_manager::input_manager::InputManager;
	use shell_input_manager::shell_input_manager::ShellInputManager;
	use text_input_manager::text_input_manager::TextInputManager;
	use dblp_input_manager::dblp_input_manager::DBLPInputManager;
	use stop_word_manager::stop_word_manager::StopWordManager;

	// Method Input
	let mut input = TextInputManager{ lines: Vec::new(), };
	input.new();

	// Stop Words
	let mut stop = StopWordManager{ stop_words: Vec::new(), };
	stop.new();

}