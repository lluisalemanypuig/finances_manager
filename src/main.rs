/*********************************************************************
 *
 * Finances Manager -- A command line utility to manage financial domestic
 * activities.
 *
 * Copyright (C) 2024
 *
 * This file is part of Finances manager. The full code is available at:
 *      https://github.com/lluisalemanypuig/finances_manager.git
 *
 * Finances Manager is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Finances Manager is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Finances Manager.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Contact:
 *
 *     Lluís Alemany Puig
 *         email: lluis.alemany.puig@gmail.com
 *         https://github.com/lluisalemanypuig
 *         lluisalemanypuig.github.io
 *
 ********************************************************************/

extern crate serde;
extern crate serde_json;

use std::io::Read;

mod date;
mod io;

mod income;
mod expense;

mod monthly_activities;
mod yearly_activities;
mod all_activities;
mod concept_types;
mod expense_summary;

mod menu_utils;
mod menu_expenses;
mod menu_expense_types;
mod menu_statistics;

type ConceptTypes = concept_types::ConceptTypes;
type AllExpenses = all_activities::AllExpenses;

fn print_main_menu() {
	println!("What menu do you want to access?");
	println!("");
	println!("    1. Expenses menu");
	println!("    2. Expenses types menu");
	println!("    3. Statistics menu");
	println!("    4. Save all data");
	println!("    0. Leave");
}

fn main_menu(all_data: &mut AllExpenses, data_dir: &String) {
	let print_function = print_main_menu;
	let min_option = 0;
	let max_option = 4;

	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {

		match option {
			1 => menu_expenses::menu(all_data),
			2 => menu_expense_types::menu(all_data),
			3 => menu_statistics::menu(all_data),
			4 => {
				io::write_all_expense_data(&data_dir, all_data).expect("Could not write data");
				for ye in all_data.expenses.iter_mut() {
					ye.set_changes(false);
				}
				all_data.expense_types.set_changes(false);
			},
			_ => println!("Nothing to do..."),
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
	let mut all_data = io::read_all_expense_data(&data_dir);
	
	println!("    Reading expense types...");
	all_data.expense_types = ConceptTypes::new_vec(
		io::read_expense_types(&data_dir),
		json.income_name
	);
	println!("    Income type name: '{}'", all_data.expense_types.income_name);

	println!("");
	println!("");
	println!("");
	println!("");
	main_menu(&mut all_data, &data_dir);
	
	io::write_all_expense_data(&data_dir, &all_data).expect("Could not write data");

}
