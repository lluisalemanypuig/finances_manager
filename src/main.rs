/*********************************************************************
 *
 * Finances Manager -- A command line utility to manage domestic financial
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

mod concepts;
mod time;
mod menus;
mod economy;

mod io;
mod utils;

use crate::economy::all_activities::AllActivities;

fn print_main_menu() {
	println!("What menu do you want to access?");
	println!("");
	println!("    1. Expenses menu");
	println!("    2. Expense concepts menu");
	println!("    3. Incomes menu");
	println!("    4. Income concepts menu");
	println!("    5. Expense statistics menu");
	println!("    6. Income statistics menu");
	println!("    7. Save changes");
	println!("    8. Force data overwrite");
	println!("    0. Leave");
}

fn main_menu(all_data: &mut AllActivities, data_dir: &String) {
	let print_function = print_main_menu;
	let min_option = 0;
	let max_option = 8;

	let mut option = menus::utils::read_option(print_function, min_option, max_option);
	while option != 0 {

		match option {
			1 => menus::activities::menu_expenses(all_data),
			2 => menus::concept_types::menu_expense_concepts(all_data),
			3 => menus::activities::menu_incomes(all_data),
			4 => menus::concept_types::menu_income_concepts(all_data),
			5 => menus::statistics::menu_expenses(all_data),
			6 => menus::statistics::menu_incomes(all_data),
			7 => {
				economy::io::write_all_data(&data_dir, all_data).expect("Could not write data");
				concepts::io::write_all_data(&data_dir, all_data).expect("Could not write data");
				
				all_data.set_changes(false);
			},
			8 => {
				// set changes to true to force overwrite
				all_data.set_changes(true);

				economy::io::write_all_data(&data_dir, all_data).expect("Could not write data");
				concepts::io::write_all_data(&data_dir, all_data).expect("Could not write data");
				
				all_data.set_changes(false);
			},
			_ => println!("Nothing to do..."),
		}
		
		option = menus::utils::read_option(print_function, min_option, max_option);
	}
	
	println!("Goodbye!");
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ProjectData {
	pub base_path: String
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
	println!("    Reading activities data...");
	let mut all_data = economy::io::read_all_activities_data(&data_dir);

	println!("    Reading expense types...");
	concepts::io::read_expense_types(&data_dir, &mut all_data);
	println!("    Reading income types...");
	concepts::io::read_income_types(&data_dir, &mut all_data);

	all_data.set_changes(false);

	println!("");
	println!("");
	println!("");
	println!("");
	main_menu(&mut all_data, &data_dir);

	economy::io::write_all_data(&data_dir, &all_data).expect("Could not write data");
	concepts::io::write_all_data(&data_dir, &all_data).expect("Could not write data");
}
