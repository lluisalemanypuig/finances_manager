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
use crate::utils;

use crate::menus;
use crate::economy;

type AllExpenses = economy::all_activities::AllActivities;

#[duplicate::duplicate_item(
	method                       get;
	[print_expense_concepts_all] [get_expense_concepts];
	[print_income_concepts_all]  [get_income_concepts];
)]
fn method(all_data: &AllExpenses) {
	println!("{}", all_data.get().get_tree());
	println!("");
}

#[duplicate::duplicate_item(
	thing       method                get_concepts           get_concepts_mut;
	["expense"] [add_expense_concept] [get_expense_concepts] [get_expense_concepts_mut];
	["income"]  [add_income_concept]  [get_income_concepts]  [get_income_concepts_mut];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Select the branch of concepts:");
	let branch = io::read_from_tree_options(&all_data.get_concepts().get_tree());
	if branch.len() == 0 { return; }

	println!("Enter the new {} concept:", thing);
	let new_concept = io::read_string();
	
	all_data.get_concepts_mut().get_tree_mut().make_subtree(&branch).insert_key(new_concept, None);
}

#[duplicate::duplicate_item(
	thing       method                   get_concepts           get_concepts_mut           iter_mut_act;
	["expense"] [rename_expense_concept] [get_expense_concepts] [get_expense_concepts_mut] [iter_mut_expenses];
	["income"]  [rename_income_concept]  [get_income_concepts]  [get_income_concepts_mut]  [iter_mut_incomes];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Select the branch of concepts (the last entered will be renamed):");
	let branch = io::read_from_tree_options(&all_data.get_concepts().get_tree());
	if branch.len() == 0 { return; }

	println!("Enter the new {} concept:", thing);
	let new_concept = io::read_string();

	let tree = all_data.get_concepts_mut().get_tree_mut();

	tree
		.make_subtree(&branch[0..branch.len()-1])
		.rename_key(branch.last().unwrap(), new_concept.clone());
	tree.normalize_tree();

	for y in all_data.iter_mut_activities() {
		for m in y.iter_mut_act() {
			for d in m.iter_mut() {
				if !utils::vector_includes(&d.concepts, &branch) { continue; }
				if d.concepts.len() >= branch.len() {
					let s = branch.len();
					d.concepts[s - 1] = new_concept.clone();
				}
			}
		}
	}
}

#[duplicate::duplicate_item(
	thing       method                   get_concepts           get_concepts_mut;
	["expense"] [remove_expense_concept] [get_expense_concepts] [get_expense_concepts_mut];
	["income"]  [remove_income_concept]  [get_income_concepts]  [get_income_concepts_mut];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Select the branch of concepts (the last entered will be removed):");
	let branch = io::read_from_tree_options(&all_data.get_concepts().get_tree());
	if branch.len() == 0 { return; }

	all_data
		.get_concepts_mut()
		.get_tree_mut()
		.get_subtree_mut(&branch[0..branch.len()-1])
		.remove_child(branch.last().unwrap());
}

#[duplicate::duplicate_item(
	thing       method;
	["expense"] [print_expense_concept_menu];
	["income"]  [print_income_concept_menu];
)]
fn method() {
	println!("Query and edit the expense concept types:");
	println!("");
	println!("    1. Show all {} concepts", thing);
	println!("    2. Add a new {} concept", thing);
	println!("    3. Rename an {} concept", thing);
	println!("    4. Remove an {} concept", thing);
	println!("    0. Leave");
}

pub fn menu_expense_concepts(all_data: &mut AllExpenses) {
	let print_function = print_expense_concept_menu;
	let min_option = 0;
	let max_option = 7;
	
	let mut option = menus::utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_expense_concepts_all(&all_data),
			2 => add_expense_concept(all_data),
			3 => rename_expense_concept(all_data),
			4 => remove_expense_concept(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menus::utils::read_option(print_function, min_option, max_option);
	}
}

pub fn menu_income_concepts(all_data: &mut AllExpenses) {
	let print_function = print_income_concept_menu;
	let min_option = 0;
	let max_option = 7;
	
	let mut option = menus::utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_income_concepts_all(&all_data),
			2 => add_income_concept(all_data),
			3 => rename_income_concept(all_data),
			4 => remove_income_concept(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menus::utils::read_option(print_function, min_option, max_option);
	}
}
