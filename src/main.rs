mod input_manager;
mod shell_input_manager;
mod text_input_manager;
mod dblp_input_manager;

fn main() {

    use input_manager::input_manager::InputManager;
	use shell_input_manager::shell_input_manager::ShellInputManager;
	use text_input_manager::text_input_manager::TextInputManager;
	use dblp_input_manager::dblp_input_manager::DBLPInputManager;

	let mut input = TextInputManager{
		lines: Vec::new(),
	};

	input.new();

	for x in 0..input.length() {
		println!("{}", input.line(x));
	}

}