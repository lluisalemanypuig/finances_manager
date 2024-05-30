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

fn print_statistics_menu() {
	println!("Perform statistics:");
	println!("");
	println!("    1. By expense type");
	println!("    2. By price");
	println!("    3. By place");
	println!("    0. Leave");
}

pub fn menu(all_data: &AllExpenses) {
	let print_function = print_statistics_menu;
	let min_option = 0;
	let max_option = 3;

	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => statistics_by_expense_type(&all_data),
			2 => statistics_by_price(&all_data),
			3 => statistics_by_place(&all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}