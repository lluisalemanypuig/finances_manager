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

#[duplicate::duplicate_item(
	method                            iterate;
	[print_expense_concept_types_all] [iter_expense_concept_types];
	[print_income_concept_types_all]  [iter_income_concept_types];
)]
fn method(all_data: &AllExpenses) {
	println!("");
	for (i, thing_type) in all_data.iterate().enumerate() {
		println!("    {i}: {thing_type}");
	}
	println!("");
}

#[duplicate::duplicate_item(
	thing         method                     get                         get_mut;
	["expense"]  [add_expense_concept_type] [get_expense_concept_types] [get_expense_concept_types_mut];
	["income"]   [add_income_concept_type]  [get_income_concept_types]  [get_income_concept_types_mut];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the new {} type:", thing);
	let new_type = io::read_string();

	if all_data.get().is_type_ok(&new_type) {
		println!("Type '{new_type}' already exists.")
	}
	else {
		all_data.get_mut().add(new_type);
	}
}

#[duplicate::duplicate_item(
	thing         method                        get                         get_mut                         iter_mut_act;
	["expense"]  [rename_expense_concept_type] [get_expense_concept_types] [get_expense_concept_types_mut] [iter_mut_expenses];
	["income"]   [rename_income_concept_type]  [get_income_concept_types]  [get_income_concept_types_mut]  [iter_mut_incomes];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the {} type to rename:", thing);
	let old_type = io::read_string();

	let idx_old_type = all_data.get().position_type(&old_type);
	
	if let Some(idx_old) = idx_old_type {
		println!("Enter the new {} type:", thing);
		let new_type = io::read_string();

		all_data.get_mut().replace(idx_old, new_type.clone());

		// replace the old type throughout the entire list of activities
		for ye in all_data.iter_mut_activities() {
			ye.set_changes(true);
			for me in ye.iter_mut_act() {
				for e in me.get_activities_mut().iter_mut().filter(|e| e.concept == old_type) {
					e.concept = new_type.clone();
				}
			}
		}
	}
	else {
		println!("Expense type '{old_type}' does not exist.")
	}
}

#[duplicate::duplicate_item(
	thing        method                        get                         get_mut;
	["expense"]  [remove_expense_concept_type] [get_expense_concept_types] [get_expense_concept_types_mut];
	["income"]   [remove_income_concept_type]  [get_income_concept_types]  [get_income_concept_types_mut];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the {} type to remove:", thing);
	let type_to_remove = io::read_string();

	if let Some(idx) = all_data.get().position_type(&type_to_remove) {
		all_data.get_mut().remove(idx);
	}
	else {
		println!("Expense type to remove '{type_to_remove}' does not exist.");
	}
}

#[duplicate::duplicate_item(
	thing        method;
	["expense"]  [print_expense_concept_types_menu];
	["income"]   [print_income_concept_types_menu];
)]
fn method() {
	println!("Query and edit the expense concept types:");
	println!("");
	println!("    1. Show all {} concept types", thing);
	println!("    2. Add a new {} concept type", thing);
	println!("    3. Rename a specific {} concept type", thing);
	println!("    4. Remove an {} concept type", thing);
	println!("    0. Leave");
}

#[duplicate::duplicate_item(
	menu                          print_menu                          print_all_types                    add_type                    rename_type                    remove_type;
	[menu_expense_concept_types]  [print_expense_concept_types_menu]  [print_expense_concept_types_all]  [add_expense_concept_type]  [rename_expense_concept_type]  [remove_expense_concept_type];
	[menu_income_concept_types]   [print_income_concept_types_menu]   [print_income_concept_types_all]   [add_income_concept_type]   [rename_income_concept_type]   [remove_income_concept_type];
)]
pub fn menu(all_data: &mut AllExpenses) {
	let print_function = print_menu;
	let min_option = 0;
	let max_option = 4;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_all_types(&all_data),
			2 => add_type(all_data),
			3 => rename_type(all_data),
			4 => remove_type(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}
