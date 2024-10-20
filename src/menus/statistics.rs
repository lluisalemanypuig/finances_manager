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

use crate::economy::expense;
use crate::economy::income;
use crate::economy::all_activities;
use crate::economy::traits::HasConcepts;

use crate::io;
use crate::menus::utils;

type Expense = expense::Expense;
type Income = income::Income;
type AllActivities = all_activities::AllActivities;
type Cell = utils::Cell;

fn sort_by_concept(a: &(Vec<String>, Cell), b: &(Vec<String>, Cell)) -> std::cmp::Ordering {
	a.0.cmp(&b.0)
}

fn sort_by_times(a: &(Vec<String>, Cell), b: &(Vec<String>, Cell)) -> std::cmp::Ordering {
	if b.1.num_times == a.1.num_times {
		return a.0.cmp(&b.0);
	}
	b.1.num_times.cmp(&a.1.num_times)
}

fn sort_by_value(a: &(Vec<String>, Cell), b: &(Vec<String>, Cell)) -> std::cmp::Ordering {
	if b.1.total_value == a.1.total_value {
		return a.0.cmp(&b.0);
	}
	b.1.total_value.total_cmp(&a.1.total_value)
}

fn concept<T: HasConcepts>(n: usize, t: &T) -> Vec<String> {
	t.get_concepts().iter().take(n).cloned().collect()
}

#[duplicate::duplicate_item(
	method                      t         title            iter_thing;
	[history_expenses_concepts] [Expense] ["Expense type"] [iter_expenses];
	[history_incomes_concepts]  [Income]  ["Income type"]  [iter_incomes];
)]
fn method<SortFunc, GroupByFunc>(
	all_data: &AllActivities,
	sort: SortFunc,
	group_by: GroupByFunc
)
where
	SortFunc: Fn( &(Vec<String>, Cell), &(Vec<String>, Cell) ) -> std::cmp::Ordering,
	GroupByFunc: Fn(&t) -> Vec<String>
{
	let mut summary: std::collections::BTreeMap<Vec<String>, Cell> = std::collections::BTreeMap::new();

	for year in all_data.iter_activities() {
		for month in year.iter_thing() {
			for exp in month.iter() {

				let group = group_by(&exp);
				match summary.get_mut(&group) {
					Some( Cell { classifier: _, num_times, total_value} ) => {
						*num_times += 1;
						*total_value += exp.price;
					},
					None => {
						summary.insert(
							group,
							Cell {
								classifier: "".to_string(),
								num_times:1,
								total_value: exp.price
							}
						);
					}
				}
			}
		}
	}

	let mut vec_summary: Vec<(Vec<String>, Cell)> = summary.into_iter().collect();
	vec_summary.sort_by(sort);

	utils::display_history_summary(
		&vec_summary, 
		title.to_string(),
		"".to_string()
	);
}

fn history_expenses_shops<F>(all_data: &AllActivities, func: F)
where
	F: Fn( &(Vec<String>, Cell), &(Vec<String>, Cell) ) -> std::cmp::Ordering
{
	let mut summary: std::collections::BTreeMap<Vec<String>, Cell> = std::collections::BTreeMap::new();

	for year in all_data.iter_activities() {
		for month in year.iter_expenses() {
			for exp in month.iter() {

				match summary.get_mut(&vec![exp.shop.clone()]) {
					Some( Cell { classifier: _, num_times, total_value} ) => {
						*num_times += 1;
						*total_value += exp.price;
					},
					None => {
						summary.insert(
							vec![exp.shop.clone()],
							Cell {
								classifier: exp.city.clone(),
								num_times: 1,
								total_value: exp.price
							}
						);
					}
				}

			}
		}
	}

	let mut vec_summary: Vec<(Vec<String>, Cell)> = summary.into_iter().collect();
	vec_summary.sort_by(func );

	utils::display_history_summary(
		&vec_summary,
		"Place".to_string(),
		"City".to_string()
	);
}

fn print_statistics_menu_expenses() {
	println!("Expense statistics:");
	println!("");
	println!("    History of expenses by type");
	println!("    1.    Sorted alphabetically");
	println!("    2.    Sorted by times");
	println!("    3.    Sorted by values");
	println!("    History of expenses by place");
	println!("    4.    Sorted alphabetically");
	println!("    5.    Sorted by times");
	println!("    6.    Sorted by value");
	println!("    0. Leave");
}

pub fn menu_expenses(all_data: &AllActivities) {
	let print_function = print_statistics_menu_expenses;
	let min_option = 0;
	let max_option = 9;

	let mut option = utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			// by type
			1 => {
				println!("How many types?");
				let num_types = io::read_int();
				history_expenses_concepts(&all_data, sort_by_concept, |e| concept(num_types, e));
			}
			2 => {
				println!("How many types?");
				let num_types = io::read_int();
				history_expenses_concepts(&all_data, sort_by_times, |e| concept(num_types, e));
			}
			3 => {
				println!("How many types?");
				let num_types = io::read_int();
				history_expenses_concepts(&all_data, sort_by_value, |e| concept(num_types, e));
			}
			
			// by place
			4 => history_expenses_shops(&all_data, sort_by_concept),
			5 => history_expenses_shops(&all_data, sort_by_times),
			6 => history_expenses_shops(&all_data, sort_by_value),

			//
			_ => println!("Nothing to do..."),
		}
		
		option = utils::read_option(print_function, min_option, max_option);
	}
}

// -----------------------------------------------------------------------------

fn history_of_from_and_place_incomes<SortFunc, GroupByFunc>(
	all_data: &AllActivities,
	title: String,
	func: SortFunc,
	group_by: GroupByFunc
)
where
	SortFunc: Fn( &(Vec<String>, Cell), &(Vec<String>, Cell) ) -> std::cmp::Ordering,
	GroupByFunc: Fn(&Income) -> Vec<String>
{
	let mut summary: std::collections::BTreeMap<Vec<String>, Cell> = std::collections::BTreeMap::new();

	for year in all_data.iter_activities() {
		for month in year.iter_incomes() {
			for inc in month.iter() {

				let key = group_by(inc);
				match summary.get_mut(&key) {
					Some( Cell { classifier: _, num_times, total_value} ) => {
						*num_times += 1;
						*total_value += inc.price;
					},
					None => {
						summary.insert(
							key,
							Cell {
								classifier: "".to_string(),
								num_times: 1,
								total_value: inc.price
							}
						);
					}
				}

			}
		}
	}

	let mut vec_summary: Vec<(Vec<String>, Cell)> = summary.into_iter().collect();
	vec_summary.sort_by(func );

	utils::display_history_summary(
		&vec_summary, 
		title,
		"".to_string()
	);
}

fn from(i: &Income) -> Vec<String> { vec![i.from.clone()] }
fn place(i: &Income) -> Vec<String> { vec![i.place.clone()] }
fn from_place(i: &Income) -> Vec<String> { vec![ i.from.clone() + " - " + &i.place.clone() ] }

fn print_statistics_menu_incomes() {
	println!("Income statistics:");
	println!("");
	println!("    History of incomes by type");
	println!("    1.    Sorted alphabetically");
	println!("    2.    Sorted by times");
	println!("    3.    Sorted by values");
	println!("    History of incomes by from");
	println!("    4.    Sorted alphabetically");
	println!("    5.    Sorted by times");
	println!("    6.    Sorted by value");
	println!("    History of incomes by place");
	println!("    7.    Sorted alphabetically");
	println!("    8.    Sorted by times");
	println!("    9.    Sorted by value");
	println!("    History of incomes by from and place");
	println!("   10.    Sorted alphabetically");
	println!("   11.    Sorted by times");
	println!("   12.    Sorted by value");
	println!("    0. Leave");
}

pub fn menu_incomes(all_data: &AllActivities) {
	let print_function = print_statistics_menu_incomes;
	let min_option = 0;
	let max_option = 15;

	let mut option = utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			// by type
			1 => {
				println!("How many types?");
				let num_types = io::read_int();
				history_incomes_concepts(&all_data, sort_by_concept, |e| concept(num_types, e));
			}
			2 => {
				println!("How many types?");
				let num_types = io::read_int();
				history_incomes_concepts(&all_data, sort_by_times, |e| concept(num_types, e));
			}
			3 => {
				println!("How many types?");
				let num_types = io::read_int();
				history_incomes_concepts(&all_data, sort_by_value, |e| concept(num_types, e));
			}

			// by from
			4 => history_of_from_and_place_incomes(&all_data, "From".to_string(), sort_by_concept, from),
			5 => history_of_from_and_place_incomes(&all_data, "From".to_string(), sort_by_times, from),
			6 => history_of_from_and_place_incomes(&all_data, "From".to_string(), sort_by_value, from),

			// by place
			7 => history_of_from_and_place_incomes(&all_data, "Place".to_string(), sort_by_concept, place),
			8 => history_of_from_and_place_incomes(&all_data, "Place".to_string(), sort_by_times, place),
			9 => history_of_from_and_place_incomes(&all_data, "Place".to_string(), sort_by_value, place),

			// by from and place
			10 => history_of_from_and_place_incomes(&all_data, "From - Place".to_string(), sort_by_concept, from_place),
			11 => history_of_from_and_place_incomes(&all_data, "From - Place".to_string(), sort_by_times, from_place),
			12 => history_of_from_and_place_incomes(&all_data, "From - Place".to_string(), sort_by_value, from_place),
			
			//
			_ => println!("Nothing to do..."),
		}
		
		option = utils::read_option(print_function, min_option, max_option);
	}
}
