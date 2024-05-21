use crate::io;
use crate::menu_utils;

use crate::all_expenses;

type AllExpenses = all_expenses::AllExpenses;

fn statistics_by_place(all_data: &AllExpenses) {
	let place: String = io::read_input_string();

	for year_data in all_data.expenses.iter() {
		println!("Data from year: {}", year_data.year);
		println!("====================");
		println!("");

		for month_data in year_data.expenses.iter() {

			menu_utils::display_and_accounting(
				all_data,
				month_data,
				|e| e.place == place
			);
		}
	}
}

fn print_statistics_menu() {
	println!("Perform statistics:");
	println!("");
	println!("    1. By place");
	println!("    0. Leave");
}

pub fn menu(all_data: &AllExpenses) {
	let print_function = print_statistics_menu;
	let min_option = 0;
	let max_option = 1;

	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => statistics_by_place(&all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}