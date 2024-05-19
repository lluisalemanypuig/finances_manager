use crate::io;

pub fn read_option<F: Fn()>(f: F, min_valid: u32, max_valid: u32) -> u32 {
	f();
	
	let mut option = io::read_input_string();
	
	while option == "" {  option = io::read_input_string(); }
	let mut option_int: u32 = option.parse().unwrap();
	
	while !(min_valid <= option_int && option_int <= max_valid) {
		f();
		
		option = io::read_input_string();
		while option == "" {  option = io::read_input_string(); }
		option_int = option.parse().unwrap();
	}
	
	option_int
}
