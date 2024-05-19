extern crate serde;
extern crate serde_json;

use std::io::Read;

mod date;
mod io;

mod expense;
mod monthly_expenses;
mod yearly_expenses;
mod all_expenses;
mod expense_types;

mod menu_utils;
mod menu_expenses;
mod menu_expense_types;

type ExpenseTypes = expense_types::ExpenseTypes;
type AllExpenses = all_expenses::AllExpenses;

fn print_main_menu() {
	println!("What menu do you want to access?");
	println!("");
	println!("    1. Expenses menu");
	println!("    2. Expenses types menu");
	println!("    3. Save all data");
	println!("    0. Leave");
}

fn main_menu(all_expense_data: &mut AllExpenses, data_dir: &String) {
	let print_function = print_main_menu;
	let min_option = 0;
	let max_option = 3;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		if option == 1 {
			menu_expenses::menu(all_expense_data);
		}
		else if option == 2 {
			menu_expense_types::menu(all_expense_data);
		}
		else if option == 3 {
			io::write_all_expense_data(&data_dir, all_expense_data).expect("Could not write data");
			for ye in all_expense_data.expenses.iter_mut() {
				ye.set_changes(false);
			}
			all_expense_data.expense_types.set_changes(false);
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
	
	println!("Goodbye!");
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ProjectData {
	pub base_path: String,
	pub income_name: String
}

fn main() {
	println!("Welcome to the expenses manager!");
	println!("");

	let mut file = std::fs::File::open("project_configuration.json").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();

	let json: ProjectData = serde_json::from_str(&data).unwrap();
	let data_dir = json.base_path;
	
	println!("Reading data from directory '{data_dir}'...");
	println!("    Reading expense data...");
	let mut all_expense_data = io::read_all_expense_data(&data_dir);
	
	println!("    Reading expense types...");
	all_expense_data.expense_types = ExpenseTypes::new_vec(
		io::read_expense_types(&data_dir),
		json.income_name
	);
	println!("    Income type name: '{}'", all_expense_data.expense_types.income_name);

	println!("");
	println!("");
	println!("");
	println!("");
	main_menu(&mut all_expense_data, &data_dir);
	
	io::write_all_expense_data(&data_dir, &all_expense_data).expect("Could not write data");

}
