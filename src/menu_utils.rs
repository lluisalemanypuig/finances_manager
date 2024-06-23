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

use crate::date;
use crate::expense;
use crate::concept_types;
use crate::monthly_activities;
use crate::activity_summary;

type Expense = expense::Expense;
type ConecptTypes = concept_types::ConceptTypes;
type MonthlyActivities = monthly_activities::MonthlyActivities<Expense>;
type ActivitySummary = activity_summary::ActivitySummary;

pub fn read_option<F: Fn()>(f: F, min_valid: u32, max_valid: u32) -> u32 {
	loop {
		f();
		let option = io::read_int();
		if min_valid <= option && option <= max_valid {
			break option;
		}
	}
}

fn center_string(s: &String, width: usize) -> String {
	let length_s = s.chars().count();
	let left_pad_size = (width - length_s)/2;
    let right_pad_size = width - left_pad_size - length_s;
	
	let left_pad: String = std::iter::repeat(" ").take(left_pad_size).collect::<String>();
	let right_pad: String = std::iter::repeat(" ").take(right_pad_size).collect::<String>();

	format!("{left_pad}{s}{right_pad}")
}

static CONCEPT_TYPE_WIDTH: usize = 15;
static PRICE_WIDTH: usize = 8;
static DATE_WIDTH: usize = 17;

pub fn display_expense_summary(summary: &ActivitySummary, pre_tab: &str) {
	let concept_type_main_divider = std::iter::repeat("—").take(CONCEPT_TYPE_WIDTH).collect::<String>();
	let concept_type_header = center_string(&"Expense type".to_string(), CONCEPT_TYPE_WIDTH);

	let price_main_divider = std::iter::repeat("—").take(PRICE_WIDTH).collect::<String>();
	let price_header = center_string(&"Price".to_string(), PRICE_WIDTH);

	let percentage_width = 10;
	let percentage_main_divider = std::iter::repeat("—").take(percentage_width).collect::<String>();
	let percentage_header = center_string(&"Percentage".to_string(), percentage_width);

	println!("");
	let tab = pre_tab.to_owned() + "    ";
	println!("{tab}+—{concept_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	println!("{tab}| {concept_type_header} | {price_header} | {percentage_header} |");
	println!("{tab}+—{concept_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	for (expense_type, value) in summary.iter_summary() {
		println!("{tab}| {:<CONCEPT_TYPE_WIDTH$} | {:>PRICE_WIDTH$.2} | {:>9.2}% |", expense_type, value, (value/summary.get_total())*100.0);
	}
	
	println!("{tab}+—{concept_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	let total_spent_msg: String = "Total spent".to_string();
	println!("{tab}| {:<CONCEPT_TYPE_WIDTH$} | {:>PRICE_WIDTH$.2} |            |", total_spent_msg, summary.get_total());
	println!("{tab}+—{concept_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	println!("");
	println!("");
}

pub fn display_and_accounting<F: Fn(&Expense) -> bool>(
	month_data: &MonthlyActivities,
	func: F
)
-> ActivitySummary
{
	let place_width =
		std::cmp::max(
			5,
			month_data
				.iter()
				.filter(|e: &&expense::Expense| func(e))
				.fold(0, |max, val| if val.place.len() > max { val.place.len() } else { max })
		);
	let place_main_divider = std::iter::repeat("—").take(place_width).collect::<String>();
	let place_mid_divider: String = std::iter::repeat("·").take(place_width).collect::<String>();
	let place_header = center_string(&"Place".to_string(), place_width);

	let city_width =
		std::cmp::max(
			5,
			month_data
				.iter()
				.filter(|e: &&expense::Expense| func(e))
				.fold(0, |max, val| if val.city.len() > max { val.city.len() } else { max })
		);
	let city_main_divider = std::iter::repeat("—").take(city_width).collect::<String>();
	let city_mid_divider: String = std::iter::repeat("·").take(city_width).collect::<String>();
	let city_header = center_string(&"City".to_string(), city_width);

	let concept_type_main_divider = std::iter::repeat("—").take(CONCEPT_TYPE_WIDTH).collect::<String>();
	let concept_type_mid_divider: String = std::iter::repeat("·").take(CONCEPT_TYPE_WIDTH).collect::<String>();
	let concept_type_header = center_string(&"Expense type".to_string(), CONCEPT_TYPE_WIDTH);

	let price_main_divider = std::iter::repeat("—").take(PRICE_WIDTH).collect::<String>();
	let price_mid_divider: String = std::iter::repeat("·").take(PRICE_WIDTH).collect::<String>();
	let price_header = center_string(&"Price".to_string(), PRICE_WIDTH);

	let date_main_divider = std::iter::repeat("—").take(DATE_WIDTH).collect::<String>();
	let date_mid_divider: String = std::iter::repeat("·").take(DATE_WIDTH).collect::<String>();
	let date_header = center_string(&"Date".to_string(), DATE_WIDTH);

	let mut summary = ActivitySummary::new();
	
	let mut first: bool = true;
	let mut some_data: bool = false;
	let mut previous_date: date::Date = date::Date { year: 1900, month: date::Month::January, day: 1};
	for (i, Expense {
		day_of_year: d,
		price: pr,
		concept: et,
		place: pl,
		city: ci,
		description: descr
	})
	in month_data.iter().filter(|e| func(e)).enumerate()
	{
		some_data = true;
		summary.add(et.clone(), *pr);

		let expense_type_text = center_string( et, CONCEPT_TYPE_WIDTH);
		let place_text = center_string( pl, place_width);
		let city_text = center_string( ci, city_width);
		if &previous_date != d {

			if first {
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{place_main_divider}—+—{city_main_divider}—+");
				println!("    | ID | {date_header} | {price_header} | {concept_type_header} | {place_header} | {city_header} | Description");
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{place_main_divider}—+—{city_main_divider}—+");
				first = false;
			}
			else {
				println!("    +————+—{date_mid_divider}—+—{price_mid_divider}—+—{concept_type_mid_divider}—+—{place_mid_divider}—+—{city_mid_divider}—+");
			}

			let date_text = center_string(&d.to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {expense_type_text} | {place_text} | {city_text} | {descr}");
			previous_date = d.clone();
		}
		else {
			let date_text = center_string(&" ".to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {expense_type_text} | {place_text} | {city_text} | {descr}");
		}
	}
	if some_data {
		println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{place_main_divider}—+—{city_mid_divider}—+");
	}

	if some_data {
		display_expense_summary(&summary, &"    ");
	}

	summary
}

pub fn display_full_summary(
	vec_summary: &Vec<(String, (u32,f32))>,
	title: String
)
{
	let title_width =
	std::cmp::max(
		title.len(),
		vec_summary
		.iter()
		.fold(0, |max, val| if val.0.len() > max { val.0.len() } else { max })
	);
	let title_main_divider = std::iter::repeat("—").take(title_width).collect::<String>();
	let title_header = center_string(&title, title_width);

	let tab = "    ";
	println!("{tab}+—{title_main_divider}—+—————————————+———————————————————+");
	println!("{tab}| {title_header} | Times found | Total money spent |");
	println!("{tab}+—{title_main_divider}—+—————————————+———————————————————+");
	for (thing, (num_times, total_value)) in vec_summary.iter() {
		let thing_text = center_string( thing, title_width);
		println!("{tab}| {thing_text} | {num_times:>11} | {total_value:>17.2} |");
	}
	println!("{tab}+—{title_main_divider}—+—————————————+———————————————————+");
}

pub fn read_correct_expense_type(expense_types: &ConecptTypes) -> Option<String> {
	loop {
		if let Some(str) = io::read_string_or_empty() {
			if expense_types.is_type_ok(&str) {
				return Some(str);
			}
		}
		else {
			return None;
		}
	}
}
