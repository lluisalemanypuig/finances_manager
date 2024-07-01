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
	method                       iterate;
	[print_expense_concepts_all] [iter_expense_subconcepts];
	[print_income_concepts_all]  [iter_income_subconcepts];
)]
fn method(all_data: &AllExpenses) {
	println!("");
	for (i, (concept_type, subtypes)) in all_data.iterate().enumerate() {
		let res = subtypes
			.iter()
			.fold(
				"".to_string(),
				|acc, s| -> String {
					if acc == "".to_string() { s.to_string() }
					else { acc + ", " + s }
				}
			);
		
		if res.len() == 0 {
			println!("    {i}: {concept_type}");
		}
		else {
			println!("    {i}: {concept_type} : {res}");
		}
	}
	println!("");
}

#[duplicate::duplicate_item(
	thing       method                get                    get_mut;
	["expense"] [add_expense_concept] [get_expense_concepts] [get_expense_concepts_mut];
	["income"]  [add_income_concept]  [get_income_concepts]  [get_income_concepts_mut];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the new {} concept:", thing);
	let new_concept = io::read_string();

	if all_data.get().has_concept(&new_concept) {
		println!("Concept '{new_concept}' already exists.")
	}
	else {
		all_data.get_mut().add_concept(new_concept);
	}
}

#[duplicate::duplicate_item(
	thing       method                   get                    get_mut;
	["expense"] [add_expense_subconcept] [get_expense_concepts] [get_expense_concepts_mut];
	["income"]  [add_income_subconcept]  [get_income_concepts]  [get_income_concepts_mut];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the current {} concept:", thing);
	let concept = io::read_from_options(all_data.get().get_concepts());

	if !all_data.get().has_concept(&concept) {
		println!("Concept '{concept}' does not exist!.");
		return;
	}
	
	println!("Enter the new {} subconcept:", thing);
	let subconcept = io::read_string();
	if all_data.get().has_subconcept(&concept, &subconcept) {
		println!("Subconcept already exists.");
		return;
	}
	all_data.get_mut().add_subconcept(concept, subconcept);
	
}

#[duplicate::duplicate_item(
	thing       method                   get                    get_mut                    iter_mut_act;
	["expense"] [rename_expense_concept] [get_expense_concepts] [get_expense_concepts_mut] [iter_mut_expenses];
	["income"]  [rename_income_concept]  [get_income_concepts]  [get_income_concepts_mut]  [iter_mut_incomes];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the {} concept to rename:", thing);
	let previous_concept = io::read_from_options(all_data.get().get_concepts());

	println!("Enter the new {} concept:", thing);
	let new_concept = io::read_string();

	all_data.get_mut().rename_concept(&previous_concept, new_concept.clone());

	// replace the old type throughout the entire list of activities
	for ye in all_data.iter_mut_activities() {
		ye.set_changes(true);
		for me in ye.iter_mut_act() {
			for e in me.get_activities_mut().iter_mut().filter(|e| e.concept == previous_concept) {
				e.concept = new_concept.clone();
			}
		}
	}
}

#[duplicate::duplicate_item(
	thing       method                      get                    get_mut                    iter_mut_act;
	["expense"] [rename_expense_subconcept] [get_expense_concepts] [get_expense_concepts_mut] [iter_mut_expenses];
	["income"]  [rename_income_subconcept]  [get_income_concepts]  [get_income_concepts_mut]  [iter_mut_incomes];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the {} concept (this will not be renamed):", thing);
	let concept = io::read_from_options(all_data.get().get_concepts());

	if all_data.get().has_concept(&concept) {
		println!("Enter the {} subconcept to be renamed:", thing);
		let old_subconcept = io::read_from_options(all_data.get().get_subconcepts(&concept));

		if all_data.get().has_subconcept(&concept, &old_subconcept) {
			println!("Enter the new {} subconcept:", thing);
			let new_subconcept = io::read_string();
	
			all_data.get_mut().rename_subconcept(&concept, &old_subconcept, new_subconcept.clone());
	
			// replace the old type throughout the entire list of activities
			for ye in all_data.iter_mut_activities() {
				ye.set_changes(true);
				for me in ye.iter_mut_act() {
					for e in me.get_activities_mut().iter_mut().filter(|e| e.sub_concept == old_subconcept) {
						e.sub_concept = new_subconcept.clone();
					}
				}
			}
		}
		else {
			println!("Subconcept '{old_subconcept}' does not exist.");
		}
	}
	else {
		println!("Concept '{concept}' does not exist.");
	}
}

#[duplicate::duplicate_item(
	thing       method                   get                    get_mut;
	["expense"] [remove_expense_concept] [get_expense_concepts] [get_expense_concepts_mut];
	["income"]  [remove_income_concept]  [get_income_concepts]  [get_income_concepts_mut];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the {} concept to remove:", thing);
	let concept_to_remove_opt = io::read_from_options_or_empty(&all_data.get().get_concepts());

	if let Some(concept) = concept_to_remove_opt {
		all_data.get_mut().remove_concept(concept);
	}
}

#[duplicate::duplicate_item(
	thing       method                      get                    get_mut;
	["expense"] [remove_expense_subconcept] [get_expense_concepts] [get_expense_concepts_mut];
	["income"]  [remove_income_subconcept]  [get_income_concepts]  [get_income_concepts_mut];
)]
fn method(all_data: &mut AllExpenses) {
	println!("Enter the {} concept (this will not be removed):", thing);
	let concept = io::read_from_options(&all_data.get().get_concepts());

	if !all_data.get().has_concept(&concept) {
		println!("Concept {concept} does not exist.");
		return;
	}

	println!("Enter the {} subconcept to remove:", thing);
	let subconcept = io::read_from_options(&all_data.get().get_subconcepts(&concept));

	all_data.get_mut().remove_subconcept(concept, subconcept);
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
	println!("    3. Add a new {} subconcept", thing);
	println!("    4. Rename an {} concept", thing);
	println!("    5. Rename an {} subconcept", thing);
	println!("    6. Remove an {} concept", thing);
	println!("    7. Remove an {} subconcept", thing);
	println!("    0. Leave");
}

pub fn menu_expense_concepts(all_data: &mut AllExpenses) {
	let print_function = print_expense_concept_menu;
	let min_option = 0;
	let max_option = 7;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_expense_concepts_all(&all_data),
			2 => add_expense_concept(all_data),
			3 => add_expense_subconcept(all_data),
			4 => rename_expense_concept(all_data),
			5 => rename_expense_subconcept(all_data),
			6 => remove_expense_concept(all_data),
			7 => remove_expense_subconcept(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}

pub fn menu_income_concepts(all_data: &mut AllExpenses) {
	let print_function = print_income_concept_menu;
	let min_option = 0;
	let max_option = 7;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_income_concepts_all(&all_data),
			2 => add_income_concept(all_data),
			3 => add_income_subconcept(all_data),
			4 => rename_income_concept(all_data),
			5 => rename_income_subconcept(all_data),
			6 => remove_income_concept(all_data),
			7 => remove_income_subconcept(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}
