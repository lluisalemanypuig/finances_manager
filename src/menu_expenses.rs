extern crate chrono;
use chrono::prelude::*;

use crate::date;
use crate::io;
use crate::menu_utils;

use crate::expense;
use crate::monthly_expenses;
use crate::yearly_expenses;
use crate::all_expenses;

type Expense = expense::Expense;
type MonthlyExpenses = monthly_expenses::MonthlyExpenses;
type YearlyExpenses = yearly_expenses::YearlyExpenses;
type AllExpenses = all_expenses::AllExpenses;

fn print_expense_data_month(all_data: &AllExpenses, month_data: &MonthlyExpenses) {
	menu_utils::display_and_accounting(all_data, month_data, |_| true);
}

fn print_expense_data_year(all_data: &AllExpenses, year_data: &YearlyExpenses) {
	println!("Data from year: {}", year_data.year);
	println!("====================");
	
	let mut total_entries = 0;
	for month_data in year_data.expenses.iter() {
		total_entries += month_data.expenses.len();
	}
	
	println!("    Found {} entries", total_entries);
	println!("");
	for month_data in year_data.expenses.iter() {
		print_expense_data_month(all_data, month_data);
	}
}

fn print_expense_data_all(all_data: &AllExpenses) {
	for year_expense in all_data.expenses.iter() {
		print_expense_data_year(all_data, &year_expense);
	}
}

fn print_expense_data_year_user(all_data: &AllExpenses) {
	println!("What year do you want to see?");
	let year: u32 = io::read_input_string().parse().unwrap();
	
	let res = all_data.get_year(&year);
	if let Some(year) = res {
		print_expense_data_year(all_data, year);
	}
	else {
		println!("Year '{year}' does not exist!");
	};
}

fn print_expense_data_month_user(all_data: &AllExpenses) {
	println!("What year and month do you want to see? Year -> Month");
	let year: u32 = io::read_input_string().parse().unwrap();
	if !all_data.has_year(&year) {
		println!("Year '{year}' does not exist.");
		return;
	}
	
	let month_opt = menu_utils::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();
	
	let res = all_data.get_month(&year, &month);
	if let Some(&ref month_data) = res {
		print_expense_data_month(all_data, &month_data);
	}
	else {
		println!("Month '{month}' does not exist in year '{year}'.");
	}
}

fn print_expense_data_current_month(all_data: &AllExpenses) {
	let now = chrono::prelude::Utc::now();
	let local_date = now.with_timezone(&chrono::prelude::Local);

	let year = local_date.year() as u32;

	let month_conv = date::Month::from_u32(local_date.month() - 1);
	if month_conv.is_none() {
		println!("Retrieval of current date failed for month '{}'.", local_date.month());
	}
	let month = month_conv.expect("This should have worked!");
	
	let res = all_data.get_month(&year, &month);
	if let Some(&ref month_data) = res {
		print_expense_data_month(all_data, &month_data);
	}
	else {
		println!("Month '{month}' does not exist in year '{year}'.");
	}
}

fn add_new_expense_with_date(all_data: &mut AllExpenses, year: u32, month: date::Month, day: u8) {
	let expense_type_opt = || -> Option<String> {
		println!("Expense Type:");
		menu_utils::read_correct_expense_type(&all_data.expense_types)
	}();
	if expense_type_opt.is_none() { return; }
	let expense_type = expense_type_opt.unwrap();

	let year_data = all_data.add_year(&year);
	let month_data = year_data.add_month(&month);

	println!("Price:");
	let price: f32 = io::read_input_string().parse().unwrap();
	
	println!("Place:");
	let place = io::read_input_string();

	println!("Description:");
	let description = io::read_input_string();

	month_data.add_expense(Expense {
		day_of_year : date::Date { year, month, day},
		price: price,
		expense_type: expense_type,
		place: place,
		description: description
	});
	year_data.set_changes(true);
}

fn add_new_expense(all_data: &mut AllExpenses) {
	println!("Year:");
	let year: u32 = io::read_input_string().parse().unwrap();

	println!("Month:");
	let month_opt = menu_utils::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	println!("Day:");
	let day: u8 = io::read_input_string().parse().unwrap();

	add_new_expense_with_date(all_data, year, month, day);
}

fn add_new_expense_today(all_data: &mut AllExpenses) {
	let now = chrono::prelude::Utc::now();
	let local_date = now.with_timezone(&chrono::prelude::Local);

	let year = local_date.year() as u32;

	let month_conv = date::Month::from_u32(local_date.month() - 1);
	if month_conv.is_none() {
		println!("Retrieval of current date failed for month '{}'.", local_date.month());
	}
	let month = month_conv.expect("This should have worked!");
	let day = local_date.day() as u8;

	add_new_expense_with_date(all_data, year, month, day);
}

fn add_new_expense_to_year_month(all_data: &mut AllExpenses) {
	println!("Year:");
	let year: u32 = io::read_input_string().parse().unwrap();

	println!("Month:");
	let month_opt = menu_utils::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	let mut changes: bool = false;
	loop {
		println!("Day:");
		let day_str = io::read_input_string();
		if day_str == "" { break; }
		let day: u8 = day_str.parse().unwrap();

		add_new_expense_with_date(all_data, year, month.clone(), day);
		changes = true;
		println!("");
	}

	all_data.get_year_mut(&year).unwrap().set_changes(changes);
}

fn edit_expense(all_data: &mut AllExpenses) {
	println!("Select year:");
	let year: u32 = io::read_input_string().parse().unwrap();
	if !all_data.has_year(&year) {
		println!("Year '{year}' does not exist.");
		return;
	}
	
	println!("Select month:");
	let month_opt = menu_utils::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	if !all_data.get_year(&year).unwrap().has_month(&month) {
		println!("Month '{month}' does not exist");
		return;
	}

	println!("Id of expense to be edited.");
	let id_expense: usize = io::read_input_string().parse().unwrap();

	let expense_type_opt = || -> Option<String> {
		let month_data = all_data.get_month(&year, &month).expect("Expected month data");
		let expense = month_data.get_expense(id_expense);
		
		println!("Expense Type: {} (leave blank to keep the value)", expense.expense_type);
		menu_utils::read_correct_expense_type(&all_data.expense_types)
	}();
	
	let year_data = all_data.add_year(&year);
	let month_data = year_data.add_month(&month);
	let expense = month_data.get_expense_mut(id_expense);
	
	if expense_type_opt.is_some() {
		expense.expense_type = expense_type_opt.unwrap();
	}

	println!("Price: {} (leave blank to keep the value)", expense.price);
	let price_str = io::read_input_string();
	if price_str != "" {
		expense.price = price_str.parse().unwrap();
	}

	println!("Place: {} (leave blank to keep the value)", expense.place);
	let place = io::read_input_string();
	if place != "" {
		expense.place = place;
	}
	println!("Description: {} (leave blank to keep the value)", expense.description);
	let description = io::read_input_string();
	if description != "" {
		expense.description = description;
	}

	year_data.set_changes(true);
}

fn remove_expense(all_data: &mut AllExpenses) {
	println!("Select year:");
	let year: u32 = io::read_input_string().parse().unwrap();
	if !all_data.has_year(&year) {
		println!("Year '{year}' does not exist.");
		return;
	}
	
	println!("Select month:");
	let month_opt = menu_utils::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	if !all_data.get_year(&year).unwrap().has_month(&month) {
		println!("Month '{month}' does not exist");
		return;
	}

	println!("Id of expense to be deleted.");
	let id_expense: usize = io::read_input_string().parse().unwrap();

	let year_data = all_data.add_year(&year);
	let month_data = year_data.add_month(&month);

	month_data.remove_expense(id_expense);
	year_data.set_changes(true);
}

fn print_expenses_menu() {
	println!("Query and edit the expenses:");
	println!("");
	println!("    1. Show all current data");
	println!("    2. Show data of a specific year");
	println!("    3. Show data of a specific month");
	println!("    4. Show data of the current month");
	println!("    5. Add another expense");
	println!("    6. Add another expense today");
	println!("    7. Add expenses to a year and month");
	println!("    8. Edit an expense");
	println!("    9. Remove an expense");
	println!("    0. Leave");
}

pub fn menu(all_data: &mut AllExpenses) {
	let print_function = print_expenses_menu;
	let min_option = 0;
	let max_option = 9;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_expense_data_all(&all_data),
			2 => print_expense_data_year_user(&all_data),
			3 => print_expense_data_month_user(&all_data),
			4 => print_expense_data_current_month(&all_data),
			5 => add_new_expense(all_data),
			6 => add_new_expense_today(all_data),
			7 => add_new_expense_to_year_month(all_data),
			8 => edit_expense(all_data),
			9 => remove_expense(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}
