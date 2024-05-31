use crate::io;
use crate::menu_utils;

use crate::all_expenses;
use crate::expense_summary;

type AllExpenses = all_expenses::AllExpenses;
type ExpenseSummary = expense_summary::ExpenseSummary;

fn statistics_by_expense_type(all_data: &AllExpenses) {
	let expense_type_opt = menu_utils::read_correct_expense_type(&all_data.expense_types);
	if expense_type_opt.is_none() { return; }
	let expense_type = expense_type_opt.unwrap();

	if !all_data.expense_types.has_expense_type(&expense_type) {
		println!("Non existent data type '{expense_type}'.");
		return;
	}

	let mut all_years = ExpenseSummary::new();
	for year_data in all_data.expenses.iter() {
		println!("Data from year: {}", year_data.year);
		println!("====================");
		println!("");

		let mut current_year = ExpenseSummary::new();

		for month_data in year_data.expenses.iter() {

			let current_month = menu_utils::display_and_accounting(
				all_data,
				month_data,
				|e| e.expense_type == expense_type
			);
			current_year.merge(current_month);
		}

		println!("This year's summary:");
		menu_utils::display_expense_summary(&current_year, all_data, &"");

		all_years.merge(current_year);
	}

	println!("Total history:");
	menu_utils::display_expense_summary(&all_years, all_data, &"");
}

fn history_of_expenses<F: FnMut( &(String,(u32,f32)), &(String,(u32,f32)) ) -> std::cmp::Ordering>
(all_data: &AllExpenses, func: F)
{

	let mut summary: std::collections::BTreeMap<String, (u32, f32)> = std::collections::BTreeMap::new();

	for year in all_data.expenses.iter() {
		for month in year.expenses.iter() {
			for exp in month.expenses.iter().filter(|e| e.expense_type != all_data.expense_types.income_name) {

				match summary.get_mut(&exp.expense_type) {
					Some( (num_times, total_value) ) => {
						*num_times += 1;
						*total_value += exp.price;
					},
					None => {
						summary.insert( exp.expense_type.clone(), (1, exp.price) );
					}
				}

			}
		}
	}

	let mut vec_summary: Vec<(String, (u32,f32))> = summary.into_iter().collect();
	vec_summary.sort_by(func );

	menu_utils::display_full_summary(&vec_summary, "Expense Type".to_string());
}

fn statistics_by_price(all_data: &AllExpenses) {
	let lower: f32 = io::read_input_string().parse().unwrap();
	let upper: f32 = io::read_input_string().parse().unwrap();

	let mut all_years = ExpenseSummary::new();
	for year_data in all_data.expenses.iter() {
		println!("Data from year: {}", year_data.year);
		println!("====================");
		println!("");

		let mut current_year = ExpenseSummary::new();

		for month_data in year_data.expenses.iter() {

			let current_month = menu_utils::display_and_accounting(
				all_data,
				month_data,
				|e| lower <= e.price && e.price <= upper
			);
			current_year.merge(current_month);
		}

		println!("This year's summary:");
		menu_utils::display_expense_summary(&current_year, all_data, &"");

		all_years.merge(current_year);
	}

	println!("Total history:");
	menu_utils::display_expense_summary(&all_years, all_data, &"");
}

fn statistics_by_place(all_data: &AllExpenses) {
	let place: String = io::read_input_string();

	let mut all_years = ExpenseSummary::new();
	for year_data in all_data.expenses.iter() {
		println!("Data from year: {}", year_data.year);
		println!("====================");
		println!("");

		let mut current_year = ExpenseSummary::new();

		for month_data in year_data.expenses.iter() {

			let current_month = menu_utils::display_and_accounting(
				all_data,
				month_data,
				|e| e.place == place
			);
			current_year.merge(current_month);
		}

		println!("This year's summary:");
		menu_utils::display_expense_summary(&current_year, all_data, &"");

		all_years.merge(current_year);
	}

	println!("Total history:");
	menu_utils::display_expense_summary(&all_years, all_data, &"");
}

fn statistics_by_place_substring(all_data: &AllExpenses) {
	let substring: String = io::read_input_string();

	let mut all_years = ExpenseSummary::new();
	for year_data in all_data.expenses.iter() {
		println!("Data from year: {}", year_data.year);
		println!("--------------------");
		println!("");

		let mut current_year = ExpenseSummary::new();

		for month_data in year_data.expenses.iter() {

			let current_month = menu_utils::display_and_accounting(
				all_data,
				month_data,
				|e| e.place.contains(&substring)
			);
			current_year.merge(current_month);
		}

		if current_year.has_data() {
			println!("This year's summary:");
			menu_utils::display_expense_summary(&current_year, all_data, &"");

			all_years.merge(current_year);
		}
	}

	if all_years.has_data() {
		println!("Total history:");
		println!("==============");
		menu_utils::display_expense_summary(&all_years, all_data, &"");
	}
}

fn history_of_places<F: FnMut( &(String,(u32,f32)), &(String,(u32,f32)) ) -> std::cmp::Ordering>
(all_data: &AllExpenses, func: F)
{

	let mut summary: std::collections::BTreeMap<String, (u32, f32)> = std::collections::BTreeMap::new();

	for year in all_data.expenses.iter() {
		for month in year.expenses.iter() {
			for exp in month.expenses.iter().filter(|e| e.expense_type != all_data.expense_types.income_name) {

				match summary.get_mut(&exp.place) {
					Some( (num_times, total_value) ) => {
						*num_times += 1;
						*total_value += exp.price;
					},
					None => {
						summary.insert( exp.place.clone(), (1, exp.price) );
					}
				}

			}
		}
	}

	let mut vec_summary: Vec<(String, (u32,f32))> = summary.into_iter().collect();
	vec_summary.sort_by(func );

	menu_utils::display_full_summary(&vec_summary, "Place".to_string());
}

fn print_statistics_menu() {
	println!("Perform statistics:");
	println!("");
	println!("    1. By expense type");
	println!("    2.     History of expenses (sorted by times)");
	println!("    3.     History of expenses (sorted by value)");
	println!("    4. By price");
	println!("    5. By place");
	println!("    6.     History of places (sorted alphabetically)");
	println!("    7.     History of places (sorted by times)");
	println!("    8.     History of places (sorted by value)");
	println!("    9.     By substring");
	println!("    0. Leave");
}

pub fn menu(all_data: &AllExpenses) {
	let print_function = print_statistics_menu;
	let min_option = 0;
	let max_option = 9;

	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => statistics_by_expense_type(&all_data),
			2 => history_of_expenses(&all_data, |a, b| b.1.0.cmp(&a.1.0)),
			3 => history_of_expenses(&all_data, |a, b| b.1.1.total_cmp(&a.1.1)),
			4 => statistics_by_price(&all_data),
			5 => statistics_by_place(&all_data),
			6 => history_of_places(&all_data, |a, b| a.0.cmp(&b.0)),
			7 => history_of_places(&all_data, |a, b| b.1.0.cmp(&a.1.0)),
			8 => history_of_places(&all_data, |a, b| b.1.1.total_cmp(&a.1.1)),
			9 => statistics_by_place_substring(&all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}