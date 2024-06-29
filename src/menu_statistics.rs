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

use crate::expense;
use crate::io;
use crate::menu_utils;

use crate::all_activities;
use crate::activity_summary;

type Expense = expense::Expense;
type AllExpenses = all_activities::AllActivities;
type ActivitySummary = activity_summary::ActivitySummary;
type Cell = menu_utils::Cell;

fn statistics_by_expense_type(all_data: &AllExpenses) {
	let expense_type_opt = io::read_from_options_or_empty(&all_data.get_expense_concepts().get_concepts());
	if expense_type_opt.is_none() { return; }
	let expense_type = expense_type_opt.unwrap();

	if !all_data.get_expense_concepts().has_concept(&expense_type) {
		println!("Non existent expense type '{expense_type}'.");
		return;
	}

	let mut all_years = ActivitySummary::new();
	for year_data in all_data.iter_activities() {
		println!("Data from year: {}", year_data.get_year());
		println!("====================");
		println!("");

		let mut current_year = ActivitySummary::new();

		for month_data in year_data.iter_expenses() {

			let current_month = menu_utils::display_and_accounting_expenses(
				month_data,
				|e| e.concept == expense_type
			);
			current_year.merge(current_month);
		}

		if current_year.has_data() {
			println!("This year's summary:");
			menu_utils::display_summary_activity(&current_year, &"");

			all_years.merge(current_year);
		}
	}

	if all_years.has_data() {
		println!("Total history:");
		menu_utils::display_summary_activity(&all_years, &"");
	}
}

fn statistics_by_price(all_data: &AllExpenses) {
	let lower: f32 = io::read_float();
	let upper: f32 = io::read_float();

	let mut all_years = ActivitySummary::new();
	for year_data in all_data.iter_activities() {
		println!("Data from year: {}", year_data.get_year());
		println!("====================");
		println!("");

		let mut current_year = ActivitySummary::new();

		for month_data in year_data.iter_expenses() {

			let current_month = menu_utils::display_and_accounting_expenses(
				month_data,
				|e| lower <= e.price && e.price <= upper
			);
			current_year.merge(current_month);
		}

		if current_year.has_data() {
			println!("This year's summary:");
			menu_utils::display_summary_activity(&current_year, &"");
			all_years.merge(current_year);
		}
	}

	if all_years.has_data() {
		println!("Total history:");
		menu_utils::display_summary_activity(&all_years, &"");
	}
}

fn statistics_by_place(all_data: &AllExpenses) {
	let place: String = io::read_string();

	let mut all_years = ActivitySummary::new();
	for year_data in all_data.iter_activities() {
		println!("Data from year: {}", year_data.get_year());
		println!("====================");
		println!("");

		let mut current_year = ActivitySummary::new();

		for month_data in year_data.iter_expenses() {

			let current_month: activity_summary::ActivitySummary = menu_utils::display_and_accounting_expenses(
				month_data,
				|e| e.shop == place
			);
			current_year.merge(current_month);
		}

		if current_year.has_data() {
			println!("This year's summary:");
			menu_utils::display_summary_activity(&current_year, &"");
			all_years.merge(current_year);
		}
	}

	if all_years.has_data() {
		println!("Total history:");
		menu_utils::display_summary_activity(&all_years, &"");
	}
}

fn statistics_by_place_substring(all_data: &AllExpenses) {
	let substring: String = io::read_string();

	let mut all_years = ActivitySummary::new();
	for year_data in all_data.iter_activities() {
		println!("Data from year: {}", year_data.get_year());
		println!("--------------------");
		println!("");

		let mut current_year = ActivitySummary::new();

		for month_data in year_data.iter_expenses() {

			let current_month = menu_utils::display_and_accounting_expenses(
				month_data,
				|e| e.shop.contains(&substring)
			);
			current_year.merge(current_month);
		}

		if current_year.has_data() {
			println!("This year's summary:");
			menu_utils::display_summary_activity(&current_year, &"");

			all_years.merge(current_year);
		}
	}

	if all_years.has_data() {
		println!("Total history:");
		println!("==============");
		menu_utils::display_summary_activity(&all_years, &"");
	}
}

fn history_of_expenses<SortFunc, ConvertFunc> (
	all_data: &AllExpenses,
	sort: SortFunc,
	convert: ConvertFunc
)
where
	SortFunc: FnMut( &(String,Cell), &(String,Cell) ) -> std::cmp::Ordering,
	ConvertFunc: Fn(&Expense) -> String
{
	let mut summary: std::collections::BTreeMap<String, Cell> = std::collections::BTreeMap::new();

	for year in all_data.iter_activities() {
		for month in year.iter_expenses() {
			for exp in month.iter() {

				let key_exp = convert(&exp);
				match summary.get_mut(&key_exp) {
					Some( Cell { city: _, num_times, total_value} ) => {
						*num_times += 1;
						*total_value += exp.price;
					},
					None => {
						summary.insert(
							key_exp,
							Cell { city: "".to_string(), num_times:1, total_value: exp.price}
						);
					}
				}
			}
		}
	}

	let mut vec_summary: Vec<(String, Cell)> = summary.into_iter().collect();
	vec_summary.sort_by(sort );

	menu_utils::display_history_summary(&vec_summary, "Expense Type".to_string(), "".to_string());
}

fn history_of_expense_places<F>(
	all_data: &AllExpenses, func: F
)
where
	F: FnMut( &(String,Cell), &(String,Cell) ) -> std::cmp::Ordering
{
	let mut summary: std::collections::BTreeMap<String, Cell> = std::collections::BTreeMap::new();

	for year in all_data.iter_activities() {
		for month in year.iter_expenses() {
			for exp in month.iter().filter(|e| e.concept != "Income") {

				match summary.get_mut(&exp.shop) {
					Some( Cell { city: _, num_times, total_value} ) => {
						*num_times += 1;
						*total_value += exp.price;
					},
					None => {
						summary.insert(
							exp.shop.clone(),
							Cell { city: exp.city.clone(), num_times: 1, total_value: exp.price }
						);
					}
				}

			}
		}
	}

	let mut vec_summary: Vec<(String, Cell)> = summary.into_iter().collect();
	vec_summary.sort_by(func );

	menu_utils::display_history_summary(&vec_summary, "Place".to_string(), "City".to_string());
}

fn print_statistics_menu_expenses() {
	println!("Expense statistics:");
	println!("");
	println!("    1. Show all expenses of a specific type");
	println!("    2. Show all expenses within a price range");
	println!("    3. Show all expenses at a place");
	println!("    4. Show all expenses at a place that contains a substring");
	println!("    History of expenses by type");
	println!("    5.    Sorted alphabetically");
	println!("    6.    Sorted by times");
	println!("    7.    Sorted by values");
	println!("    History of expenses by type and subtype");
	println!("    8.    Sorted alphabetically");
	println!("    9.    Sorted by times");
	println!("   10.    Sorted by values");
	println!("    History of expenses by place");
	println!("   11.    Sorted alphabetically");
	println!("   12.    Sorted by times");
	println!("   13.    Sorted by value");
	println!("    0. Leave");
}

fn sort_by_concept_alphabetically(a: &(String, Cell), b: &(String, Cell)) -> std::cmp::Ordering {
	a.0.cmp(&b.0)
}

fn sort_by_times(a: &(String, Cell), b: &(String, Cell)) -> std::cmp::Ordering {
	if b.1.num_times == a.1.num_times {
		return a.0.cmp(&b.0);
	}
	b.1.num_times.cmp(&a.1.num_times)
}

fn sort_by_value(a: &(String, Cell), b: &(String, Cell)) -> std::cmp::Ordering {
	if b.1.total_value == a.1.total_value {
		return a.0.cmp(&b.0);
	}
	b.1.total_value.total_cmp(&a.1.total_value)
}

fn concept(e: &Expense) -> String { e.concept.clone() }
fn concept_subconcept(e: &Expense) -> String {
	let sub_concept;
	if e.sub_concept != "".to_string() {
		sub_concept = " - ".to_owned() + &e.sub_concept.clone();
	}
	else {
		sub_concept = "".to_string();
	}
	e.concept.clone() + &sub_concept
}

pub fn menu_expenses(all_data: &AllExpenses) {
	let print_function = print_statistics_menu_expenses;
	let min_option = 0;
	let max_option = 13;

	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => statistics_by_expense_type(&all_data),
			2 => statistics_by_price(&all_data),
			3 => statistics_by_place(&all_data),
			4 => statistics_by_place_substring(&all_data),
			// by type
			5 => history_of_expenses(&all_data, sort_by_concept_alphabetically, concept),
			6 => history_of_expenses(&all_data, sort_by_times, concept),
			7 => history_of_expenses(&all_data, sort_by_value, concept),
			// by type and subtype
			8 => history_of_expenses(&all_data, sort_by_concept_alphabetically, concept_subconcept),
			9 => history_of_expenses(&all_data, sort_by_times, concept_subconcept),
			10 => history_of_expenses(&all_data, sort_by_value, concept_subconcept),
			// by place
			11 => history_of_expense_places(&all_data, sort_by_concept_alphabetically),
			12 => history_of_expense_places(&all_data, sort_by_times),
			13 => history_of_expense_places(&all_data, sort_by_value),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}
