mod input_manager;
mod shell_input_manager;

fn main() {

    use input_manager::imput_manager::InputManager;
	use shell_input_manager::shell_imput_manager::ShellInputManager;

	let mut a = ShellInputManager{};
	a.new();
	println!("{}",  a.line(0));
	println!("{}", a.length());

}