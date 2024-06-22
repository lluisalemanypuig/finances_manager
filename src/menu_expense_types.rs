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

use crate::io;
use crate::menu_utils;

use crate::all_activities;

type AllExpenses = all_activities::AllActivities;

fn print_expense_types_all(all_data: &AllExpenses) {
	println!("");
	for (i, expense_type) in all_data.get_expense_types().get_types().iter().enumerate() {
		println!("    {i}: {expense_type}");
	}
	println!("");
}

fn add_expense_type(all_data: &mut AllExpenses) {
	println!("Enter the new type of expense:");
	let new_expense = io::read_string();

	if all_data.get_expense_types().is_type_ok(&new_expense) {
		println!("Expense type '{new_expense}' already exists.")
	}
	else {
		all_data.get_expense_types_mut().add(new_expense);
	}
}

fn rename_expense_type(all_data: &mut AllExpenses) {
	println!("Enter the type of expense to rename:");
	let old_expense = io::read_string();

	let idx_old_expense = all_data.get_expense_types().position_type(&old_expense);
	
	if let Some(idx_old) = idx_old_expense {
		println!("Enter the new type of expense:");
		let new_expense = io::read_string();

		all_data.get_expense_types_mut().replace(idx_old, new_expense.clone());

		// replace the old expense type throughout the entire list of expenses
		for ye in all_data.get_activities_mut().iter_mut() {
			ye.set_changes(true);
			for me in ye.get_expenses_mut().get_activities_mut().iter_mut() {
				for e in me.get_activities_mut().iter_mut().filter(|e| e.expense_type == old_expense) {
					e.expense_type = new_expense.clone();
				}
			}
		}
	}
	else {
		println!("Expense type '{old_expense}' does not exist.")
	}
}

fn remove_expense_type(all_data: &mut AllExpenses) {
	println!("Enter the type of expense to remove:");
	let expense_to_remove = io::read_string();

	if let Some(idx) = all_data.get_expense_types().position_type(&expense_to_remove) {
		all_data.get_expense_types_mut().remove(idx);
	}
	else {
		println!("Expense type to remove '{expense_to_remove}' does not exist.");
	}
}

fn print_expense_type_menu() {
	println!("Query and edit the expense types:");
	println!("");
	println!("    1. Show all expense types");
	println!("    2. Add a new expense type");
	println!("    3. Rename a specific expense type");
	println!("    4. Remove an expense type");
	println!("    0. Leave");
}

pub fn menu(all_data: &mut AllExpenses) {
	let print_function = print_expense_type_menu;
	let min_option = 0;
	let max_option = 4;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_expense_types_all(&all_data),
			2 => add_expense_type(all_data),
			3 => rename_expense_type(all_data),
			4 => remove_expense_type(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
	
	println!("Goodbye!");
}
