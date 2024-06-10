/*********************************************************************
 *
 * Finances Manager -- A command line utility to manage financial domestic
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

extern crate chrono;
use chrono::prelude::*;

use crate::date;
use crate::io;
use crate::io::read_float_or_empty;
use crate::menu_utils;

use crate::expense;
use crate::monthly_activities;
use crate::yearly_activities;
use crate::all_activities;
use crate::expense_summary;

type Expense = expense::Expense;
type MonthlyActivities = monthly_activities::MonthlyActivities;
type YearlyActivities = yearly_activities::YearlyActivities;
type AllExpenses = all_activities::AllActivities;

type ExpenseSummary = expense_summary::ExpenseSummary;

fn print_expense_data_month(month_data: &MonthlyActivities)
-> ExpenseSummary
{
	menu_utils::display_and_accounting(month_data, |_| true)
}

fn print_expense_data_year(year_data: &YearlyActivities)
-> ExpenseSummary
{
	println!("Data from year: {}", year_data.get_year());
	println!("--------------------");
	
	let mut total_entries = 0;
	for month_data in year_data.activities.iter() {
		total_entries += month_data.expenses.len();
	}
	
	let mut current_year = ExpenseSummary::new();

	println!("    Found {} entries", total_entries);
	println!("");
	for month_data in year_data.activities.iter() {
		let current_month = print_expense_data_month(month_data);
		current_year.merge(current_month);
	}

	println!("This year's summary:");
	menu_utils::display_expense_summary(&current_year, &"");

	current_year
}

fn print_expense_data_all(all_data: &AllExpenses) {
	let mut all_years = ExpenseSummary::new();

	for year_expense in all_data.activities.iter() {
		let current_year = print_expense_data_year(&year_expense);
		all_years.merge(current_year);
	}

	println!("Total history:");
	println!("==============");
	menu_utils::display_expense_summary(&all_years, &"");
}

fn print_expense_data_year_user(all_data: &AllExpenses) {
	println!("What year do you want to see?");
	let year: u32 = io::read_int();
	
	let res = all_data.get_year(&year);
	if let Some(year) = res {
		print_expense_data_year(year);
	}
	else {
		println!("Year '{year}' does not exist!");
	};
}

fn print_expense_data_year_current(all_data: &AllExpenses) {
	let now = chrono::prelude::Utc::now();
	let local_date = now.with_timezone(&chrono::prelude::Local);

	let year = local_date.year() as u32;

	let res = all_data.get_year(&year);
	if let Some(year) = res {
		print_expense_data_year(year);
	}
	else {
		println!("Year '{year}' does not exist!");
	};
}

fn print_expense_data_month_user(all_data: &AllExpenses) {
	println!("What year and month do you want to see? Year -> Month");
	let year: u32 = io::read_int();
	if !all_data.has_year(&year) {
		println!("Year '{year}' does not exist.");
		return;
	}
	
	let month_opt = io::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();
	
	let res = all_data.get_month(&year, &month);
	if let Some(&ref month_data) = res {
		print_expense_data_month(&month_data);
	}
	else {
		println!("Month '{month}' does not exist in year '{year}'.");
	}
}

fn print_expense_data_month_current(all_data: &AllExpenses) {
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
		print_expense_data_month(&month_data);
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
	let price: f32 = io::read_float();
	
	println!("Place:");
	let place = io::read_string();

	println!("Description:");
	let description = match io::read_string_or_empty() {
		Some(str) => str,
		None => "".to_string()
	};

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
	let year: u32 = io::read_int();

	println!("Month:");
	let month_opt = io::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	println!("Day:");
	let day: u8 = io::read_int();

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
	let year: u32 = io::read_int();

	println!("Month:");
	let month_opt = io::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	let mut changes: bool = false;
	loop {
		println!("Day:");
		match io::read_int_or_empty::<u8>() {
			Some(day) => {
				add_new_expense_with_date(all_data, year, month.clone(), day);
				changes = true;
				println!("");
			},
			None => {
				break;
			}
		}
	}

	all_data.get_year_mut(&year).unwrap().set_changes(changes);
}

fn edit_expense(all_data: &mut AllExpenses) {
	println!("Select year:");
	let year: u32 = io::read_int();
	if !all_data.has_year(&year) {
		println!("Year '{year}' does not exist.");
		return;
	}
	
	println!("Select month:");
	let month_opt = io::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	if !all_data.get_year(&year).unwrap().has_month(&month) {
		println!("Month '{month}' does not exist");
		return;
	}

	{
	let year_data = all_data.get_year(&year).unwrap();
	let month_data = year_data.get_month(&month).unwrap();
	menu_utils::display_and_accounting(month_data, |_| true);
	}

	println!("Id of expense to be edited.");
	let id_expense_opt = io::read_int_or_empty::<usize>();
	if id_expense_opt.is_none() { return; }
	let id_expense = id_expense_opt.unwrap();

	let expense_type_opt: Option<String>;
	{
		let month_data = all_data.get_month(&year, &month).expect("Expected month data");
		let expense = month_data.get_expense(id_expense);
		println!("Expense Type: {} (leave blank to keep the value)", expense.expense_type);
		expense_type_opt = menu_utils::read_correct_expense_type(&all_data.expense_types)
	}
	
	let year_data = all_data.add_year(&year);
	let month_data = year_data.add_month(&month);
	let expense = month_data.get_expense_mut(id_expense);
	
	if expense_type_opt.is_some() {
		expense.expense_type = expense_type_opt.unwrap();
	}

	println!("Price: {} (leave blank to keep the value)", expense.price);
	if let Some(price) = read_float_or_empty::<f32>() {
		expense.price = price;
	}

	println!("Place: {} (leave blank to keep the value)", expense.place);
	if let Some(place) = io::read_string_or_empty() {
		expense.place = place;
	}

	println!("Description: {} (leave blank to keep the value)", expense.description);
	if let Some(value) = io::read_string_or_empty() {
		expense.description = value;
	}
	
	year_data.set_changes(true);
}

fn remove_expense(all_data: &mut AllExpenses) {
	println!("Select year:");
	let year: u32 = io::read_int();
	if !all_data.has_year(&year) {
		println!("Year '{year}' does not exist.");
		return;
	}
	
	println!("Select month:");
	let month_opt = io::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	if !all_data.get_year(&year).unwrap().has_month(&month) {
		println!("Month '{month}' does not exist");
		return;
	}

	println!("Id of expense to be deleted.");
	if let Some(id_expense) = io::read_int_or_empty::<usize>() {
		let year_data = all_data.add_year(&year);
		let month_data = year_data.add_month(&month);

		month_data.remove_expense(id_expense);
		year_data.set_changes(true);
	}
}

fn print_expenses_menu() {
	println!("Query and edit the expenses:");
	println!("");
	println!("     1. Show all current data");
	println!("     2. Show data of a year");
	println!("     3.     Show data of the current year");
	println!("     4. Show data of a month");
	println!("     5.     Show data of the current month");
	println!("     6. Add another expense");
	println!("     7.     Add another expense today");
	println!("     8.     Add expenses to a year and month");
	println!("     9. Edit an expense");
	println!("    10. Remove an expense");
	println!("     0. Leave");
}

pub fn menu(all_data: &mut AllExpenses) {
	let print_function = print_expenses_menu;
	let min_option = 0;
	let max_option = 10;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_expense_data_all(&all_data),
			2 => print_expense_data_year_user(&all_data),
			3 => print_expense_data_year_current(&all_data),
			4 => print_expense_data_month_user(&all_data),
			5 => print_expense_data_month_current(&all_data),
			6 => add_new_expense(all_data),
			7 => add_new_expense_today(all_data),
			8 => add_new_expense_to_year_month(all_data),
			9 => edit_expense(all_data),
			10 => remove_expense(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}
