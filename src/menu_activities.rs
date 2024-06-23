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

extern crate chrono;
use chrono::prelude::*;

use crate::date;
use crate::io;
use crate::io::read_float_or_empty;
use crate::menu_utils;

use crate::expense;
use crate::income;
use crate::monthly_activities;
use crate::yearly_activities;
use crate::all_activities;
use crate::activity_summary;

type Expense = expense::Expense;
type Income = income::Income;
type MonthlyActivities<T> = monthly_activities::MonthlyActivities<T>;
type YearlyActivities = yearly_activities::YearlyActivities;
type AllActivities = all_activities::AllActivities;

type ActivitySummary = activity_summary::ActivitySummary;

#[duplicate::duplicate_item(
	method                      func                              activity;
	[print_data_month_expenses] [display_and_accounting_expenses] [Expense];
	[print_data_month_incomes]  [display_and_accounting_incomes]  [Income];
)]
fn method(month_data: &MonthlyActivities<activity>)
-> ActivitySummary
{
	menu_utils::func(month_data, |_| true)
}

#[duplicate::duplicate_item(
	method                     iterate         print;
	[print_data_year_expenses] [iter_expenses] [print_data_month_expenses];
	[print_data_year_incomes]  [iter_incomes]  [print_data_month_incomes];
)]
fn method(year_data: &YearlyActivities)
-> ActivitySummary
{
	println!("Data from year: {}", year_data.get_year());
	println!("--------------------");
	
	let mut total_entries = 0;
	for month_data in year_data.iterate() {
		total_entries += month_data.size();
	}
	
	let mut current_year = ActivitySummary::new();

	println!("    Found {} entries", total_entries);
	println!("");
	for month_data in year_data.iterate() {
		let current_month = print(month_data);
		current_year.merge(current_month);
	}

	if current_year.has_data() {
		println!("This year's summary:");
		menu_utils::display_summary_activity(&current_year, &"");
	}

	current_year
}

#[duplicate::duplicate_item(
	method                    print;
	[print_data_all_expenses] [print_data_year_expenses];
	[print_data_all_incomes]  [print_data_year_incomes];
)]
fn method(all_data: &AllActivities) {
	let mut all_years = ActivitySummary::new();

	for year_expense in all_data.iter_activities() {
		let current_year = print(&year_expense);
		all_years.merge(current_year);
	}

	println!("Total history:");
	println!("==============");
	menu_utils::display_summary_activity(&all_years, &"");
}

#[duplicate::duplicate_item(
	method                          print;
	[print_data_year_user_expenses] [print_data_year_expenses];
	[print_data_year_user_incomes]  [print_data_year_incomes];
)]
fn method(all_data: &AllActivities) {
	println!("What year do you want to see?");
	let year: u32 = io::read_int();
	
	let res = all_data.get_year(&year);
	if let Some(year) = res {
		print(year);
	}
	else {
		println!("Year '{year}' does not exist!");
	};
}

#[duplicate::duplicate_item(
	method                             print;
	[print_data_year_current_expenses] [print_data_year_expenses];
	[print_data_year_current_incomes]  [print_data_year_incomes];
)]
fn method(all_data: &AllActivities) {
	let now = chrono::prelude::Utc::now();
	let local_date = now.with_timezone(&chrono::prelude::Local);

	let year = local_date.year() as u32;

	let res = all_data.get_year(&year);
	if let Some(year) = res {
		print(year);
	}
	else {
		println!("Year '{year}' does not exist!");
	};
}

#[duplicate::duplicate_item(
	method                           get                  print;
	[print_data_month_user_expenses] [get_month_expenses] [print_data_month_expenses];
	[print_data_month_user_incomes]  [get_month_incomes]  [print_data_month_incomes];
)]
fn method(all_data: &AllActivities) {
	println!("What year and month do you want to see? Year -> Month");
	let year: u32 = io::read_int();
	if !all_data.has_year(&year) {
		println!("Year '{year}' does not exist.");
		return;
	}
	
	let month_opt = io::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();
	
	let res = all_data.get(&year, &month);
	if let Some(&ref month_data) = res {
		print(&month_data);
	}
	else {
		println!("Month '{month}' does not exist in year '{year}'.");
	}
}

#[duplicate::duplicate_item(
	method                              get                  print;
	[print_data_month_current_expenses] [get_month_expenses] [print_data_month_expenses];
	[print_data_month_current_incomes]  [get_month_incomes]  [print_data_month_incomes];
)]
fn method(all_data: &AllActivities) {
	let now = chrono::prelude::Utc::now();
	let local_date = now.with_timezone(&chrono::prelude::Local);

	let year = local_date.year() as u32;

	let month_conv = date::Month::from_u32(local_date.month() - 1);
	if month_conv.is_none() {
		println!("Retrieval of current date failed for month '{}'.", local_date.month());
	}
	let month = month_conv.expect("This should have worked!");
	
	let res = all_data.get(&year, &month);
	if let Some(&ref month_data) = res {
		print(&month_data);
	}
	else {
		println!("Month '{month}' does not exist in year '{year}'.");
	}
}

fn add_new_with_date_expense(all_data: &mut AllActivities, year: u32, month: date::Month, day: u8) {
	let expense_type_opt = || -> Option<String> {
		println!("Expense Type:");
		menu_utils::read_correct_concept_type(&all_data.get_expense_concept_types())
	}();
	if expense_type_opt.is_none() { return; }
	let expense_type = expense_type_opt.unwrap();

	let year_data = all_data.add_year(year);
	let month_data = year_data.get_expenses_mut().add(&month);

	println!("Price:");
	let price: f32 = io::read_float();
	
	println!("Place:");
	let place = io::read_string();

	println!("City:");
	let city = io::read_string();

	println!("Description:");
	let description = match io::read_string_or_empty() {
		Some(str) => str,
		None => "".to_string()
	};

	month_data.push(Expense {
		day_of_year : date::Date { year, month, day},
		price: price,
		concept: expense_type,
		place: place,
		city: city,
		description: description
	});
}

fn add_new_with_date_income(all_data: &mut AllActivities, year: u32, month: date::Month, day: u8) {
	let expense_type_opt = || -> Option<String> {
		println!("Income Type:");
		menu_utils::read_correct_concept_type(&all_data.get_income_concept_types())
	}();
	if expense_type_opt.is_none() { return; }
	let expense_type = expense_type_opt.unwrap();

	let year_data = all_data.add_year(year);
	let month_data = year_data.get_incomes_mut().add(&month);

	println!("Price:");
	let price: f32 = io::read_float();
	
	println!("From:");
	let from = io::read_string();

	println!("Place:");
	let place = io::read_string();

	println!("Description:");
	let description = match io::read_string_or_empty() {
		Some(str) => str,
		None => "".to_string()
	};

	month_data.push(Income {
		day_of_year : date::Date { year, month, day},
		price: price,
		concept: expense_type,
		from: from,
		place: place,
		description: description
	});
}

#[duplicate::duplicate_item(
	method            add;
	[add_new_expense] [add_new_with_date_expense];
	[add_new_income]  [add_new_with_date_income];
)]
fn method(all_data: &mut AllActivities) {
	println!("Year:");
	let year: u32 = io::read_int();

	println!("Month:");
	let month_opt = io::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	println!("Day:");
	let day: u8 = io::read_int();

	add(all_data, year, month, day);
}

#[duplicate::duplicate_item(
	method                  add;
	[add_new_today_expense] [add_new_with_date_expense];
	[add_new_today_income]  [add_new_with_date_income];
)]
fn method(all_data: &mut AllActivities) {
	let now = chrono::prelude::Utc::now();
	let local_date = now.with_timezone(&chrono::prelude::Local);

	let year = local_date.year() as u32;

	let month_conv = date::Month::from_u32(local_date.month() - 1);
	if month_conv.is_none() {
		println!("Retrieval of current date failed for month '{}'.", local_date.month());
	}
	let month = month_conv.expect("This should have worked!");
	let day = local_date.day() as u8;

	add(all_data, year, month, day);
}

#[duplicate::duplicate_item(
	method                       add;
	[add_new_year_month_expense] [add_new_with_date_expense];
	[add_new_year_month_income]  [add_new_with_date_income];
)]
fn method(all_data: &mut AllActivities) {
	println!("Year:");
	let year: u32 = io::read_int();

	println!("Month:");
	let month_opt = io::read_correct_month();
	if month_opt.is_none() { return; }
	let month = month_opt.unwrap();

	loop {
		println!("Day:");
		match io::read_int_or_empty::<u8>() {
			Some(day) => {
				add(all_data, year, month.clone(), day);
				println!("");
			},
			None => {
				break;
			}
		}
	}
}

fn edit_expense(all_data: &mut AllActivities) {
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

	if !all_data.get_year(&year).unwrap().get_expenses().has_month(&month) {
		println!("Month '{month}' does not exist");
		return;
	}

	{
	let year_data = all_data.get_year(&year).unwrap();
	let month_data = year_data.get_expenses().get_month(&month).unwrap();
	menu_utils::display_and_accounting_expenses(month_data, |_| true);
	}

	println!("Id of expense to be edited.");
	let id_expense_opt = io::read_int_or_empty::<usize>();
	if id_expense_opt.is_none() { return; }
	let id_expense = id_expense_opt.unwrap();

	let expense_type_opt: Option<String>;
	{
		let month_data = all_data.get_month_expenses(&year, &month).expect("Expected month data");
		let expense = month_data.get(id_expense);
		println!("Expense Type: {} (leave blank to keep the value)", expense.concept);
		expense_type_opt = menu_utils::read_correct_concept_type(&all_data.get_expense_concept_types())
	}
	
	let year_data = all_data.add_year(year);
	let month_data = year_data.get_expenses_mut().add(&month);
	let expense = month_data.get_mut(id_expense);
	
	if expense_type_opt.is_some() {
		expense.concept = expense_type_opt.unwrap();
	}

	println!("Price: {} (leave blank to keep the value)", expense.price);
	if let Some(price) = read_float_or_empty::<f32>() {
		expense.price = price;
	}

	println!("Place: {} (leave blank to keep the value)", expense.place);
	if let Some(place) = io::read_string_or_empty() {
		expense.place = place;
	}

	println!("City: {} (leave blank to keep the value)", expense.city);
	if let Some(city) = io::read_string_or_empty() {
		expense.city = city;
	}

	println!("Description: {} (leave blank to keep the value)", expense.description);
	if let Some(value) = io::read_string_or_empty() {
		expense.description = value;
	}
	
	year_data.set_changes(true);
}

fn edit_income(all_data: &mut AllActivities) {
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

	if !all_data.get_year(&year).unwrap().get_incomes().has_month(&month) {
		println!("Month '{month}' does not exist");
		return;
	}

	{
	let year_data = all_data.get_year(&year).unwrap();
	let month_data = year_data.get_incomes().get_month(&month).unwrap();
	menu_utils::display_and_accounting_incomes(month_data, |_| true);
	}

	println!("Id of expense to be edited.");
	let id_expense_opt = io::read_int_or_empty::<usize>();
	if id_expense_opt.is_none() { return; }
	let id_expense = id_expense_opt.unwrap();

	let expense_type_opt: Option<String>;
	{
		let month_data = all_data.get_month_incomes(&year, &month).expect("Expected month data");
		let expense = month_data.get(id_expense);
		println!("Expense Type: {} (leave blank to keep the value)", expense.concept);
		expense_type_opt = menu_utils::read_correct_concept_type(&all_data.get_income_concept_types())
	}
	
	let year_data = all_data.add_year(year);
	let month_data = year_data.get_incomes_mut().add(&month);
	let income = month_data.get_mut(id_expense);
	
	if expense_type_opt.is_some() {
		income.concept = expense_type_opt.unwrap();
	}

	println!("Price: {} (leave blank to keep the value)", income.price);
	if let Some(price) = read_float_or_empty::<f32>() {
		income.price = price;
	}

	println!("From: {} (leave blank to keep the value)", income.from);
	if let Some(from) = io::read_string_or_empty() {
		income.from = from;
	}

	println!("Place: {} (leave blank to keep the value)", income.place);
	if let Some(place) = io::read_string_or_empty() {
		income.place = place;
	}

	println!("Description: {} (leave blank to keep the value)", income.description);
	if let Some(value) = io::read_string_or_empty() {
		income.description = value;
	}
	
	year_data.set_changes(true);
}

#[duplicate::duplicate_item(
	method           get            get_mut;
	[remove_expense] [get_expenses] [get_expenses_mut];
	[remove_income]  [get_incomes]  [get_incomes_mut];
)]
fn method(all_data: &mut AllActivities) {
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

	if !all_data.get_year(&year).unwrap().get().has_month(&month) {
		println!("Month '{month}' does not exist");
		return;
	}

	println!("Id of expense to be deleted.");
	if let Some(id_expense) = io::read_int_or_empty::<usize>() {
		let year_data = all_data.add_year(year);
		let month_data = year_data.get_mut().add(&month);

		month_data.remove(id_expense);
		year_data.set_changes(true);
	}
}

#[duplicate::duplicate_item(
	method                thing;
	[print_menu_expenses] ["expense"];
	[print_menu_income]   ["income"];
)]
fn method() {
	println!("Query and edit the expenses:");
	println!("");
	println!("     1. Show all current data");
	println!("     2. Show data of a year");
	println!("     3.     Show data of the current year");
	println!("     4. Show data of a month");
	println!("     5.     Show data of the current month");
	println!("     6. Add another {}", thing);
	println!("     7.     Add another {} today", thing);
	println!("     8.     Add {}s to a year and month", thing);
	println!("     9. Edit an {}", thing);
	println!("    10. Remove an {}", thing);
	println!("     0. Leave");
}

pub fn menu_expenses(all_data: &mut AllActivities) {
	let print_function = print_menu_expenses;
	let min_option = 0;
	let max_option = 10;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_data_all_expenses(&all_data),
			2 => print_data_year_user_expenses(&all_data),
			3 => print_data_year_current_expenses(&all_data),
			4 => print_data_month_user_expenses(&all_data),
			5 => print_data_month_current_expenses(&all_data),
			6 => add_new_expense(all_data),
			7 => add_new_today_expense(all_data),
			8 => add_new_year_month_expense(all_data),
			9 => edit_expense(all_data),
			10 => remove_expense(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}

pub fn menu_incomes(all_data: &mut AllActivities) {
	let print_function = print_menu_income;
	let min_option = 0;
	let max_option = 10;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => print_data_all_incomes(&all_data),
			2 => print_data_year_user_incomes(&all_data),
			3 => print_data_year_current_incomes(&all_data),
			4 => print_data_month_user_incomes(&all_data),
			5 => print_data_month_current_incomes(&all_data),
			6 => add_new_income(all_data),
			7 => add_new_today_income(all_data),
			8 => add_new_year_month_income(all_data),
			9 => edit_income(all_data),
			10 => remove_income(all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}