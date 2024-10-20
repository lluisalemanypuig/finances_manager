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
use crate::income;
use crate::monthly_activities;
use crate::activity_summary;
use crate::traits::AsReferences;
use crate::traits::HasConcepts;

type Expense = expense::Expense;
type Income = income::Income;
type MonthlyActivities<T> = monthly_activities::MonthlyActivities<T>;
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

fn left_justify_string(s: &String, width: usize) -> String {
	let length_s = s.chars().count();
	let right_pad_size = width - length_s;
	let right_pad: String = std::iter::repeat(" ").take(right_pad_size).collect::<String>();
	format!("{s}{right_pad}")
}

fn center_string(s: &String, width: usize) -> String {
	let length_s = s.chars().count();
	let left_pad_size = (width - length_s)/2;
	let right_pad_size = width - left_pad_size - length_s;

	let left_pad: String = std::iter::repeat(" ").take(left_pad_size).collect::<String>();
	let right_pad: String = std::iter::repeat(" ").take(right_pad_size).collect::<String>();

	format!("{left_pad}{s}{right_pad}")
}

fn types_text(ts: &Vec<String>, column_widths: &Vec<usize>, sep: &str) -> String {
	let mut justified_types: Vec<String> = vec![];
	justified_types.resize(column_widths.len(), "".to_string());

	for i in 0..justified_types.len() {
		if i < ts.len() {
			justified_types[i] = left_justify_string(&ts[i], column_widths[i]);
		}
		else {
			justified_types[i] = left_justify_string(&"".to_string(), column_widths[i]);
		}
	}
	
	justified_types.join(sep)
}

fn calculate_type_columns_width<T, F>(month_data: &MonthlyActivities<T>, func: &F) -> Vec<usize>
where
	T: HasConcepts + Ord + Eq + PartialEq + AsReferences<T>,
	F: Fn(&T) -> bool
{
	let number_of_types =
		month_data
			.iter()
			.filter(|d: &&T| func(d))
			.map(|d| -> usize { d.get_concepts().len() })
			.max()
			.unwrap_or(0);

	let mut maxs: Vec<usize> = vec![];
	maxs.resize(number_of_types, 0);

	for d in month_data.iter() {
		for j in 0..d.get_concepts().len() {
			maxs[j] = std::cmp::max(maxs[j], d.get_concepts()[j].chars().count())
		}
	}

	maxs
}

// the concept type width is 5 (and not 4) due to the length of the word "Total"
static CONCEPT_TYPE_WIDTH: usize = 5;
static PRICE_WIDTH: usize = 8;
static DATE_WIDTH: usize = 17;
static CONCEPT_TYPE_SEPARATOR: &str = " ; ";

pub fn display_summary_activity(summary: &ActivitySummary, pre_tab: &str) {
	let type_columns_width = summary.get_type_max_widths();
	let type_column_width: usize =
		type_columns_width.iter().sum::<usize>()
		+
		(type_columns_width.len() - 1)*CONCEPT_TYPE_SEPARATOR.len()
		;

	let type_width = std::cmp::max(CONCEPT_TYPE_WIDTH, type_column_width);
	let type_main_divider = std::iter::repeat("—").take(type_width).collect::<String>();
	let type_header = center_string(&"Type".to_string(), type_width);

	let price_main_divider = std::iter::repeat("—").take(PRICE_WIDTH).collect::<String>();
	let price_header = center_string(&"Price".to_string(), PRICE_WIDTH);

	let percentage_width = 10;
	let percentage_main_divider = std::iter::repeat("—").take(percentage_width).collect::<String>();
	let percentage_header = center_string(&"Percentage".to_string(), percentage_width);

	let tab = pre_tab.to_owned() + "    ";
	println!("{tab}+—{type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	println!("{tab}| {type_header} | {price_header} | {percentage_header} |");
	println!("{tab}+—{type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	for (v, value) in summary.iter_summary() {
		println!(
			"{tab}| {:<type_width$} | {:>PRICE_WIDTH$.2} | {:>9.2}% |",
			types_text(v, &type_columns_width, CONCEPT_TYPE_SEPARATOR),
			value,
			(value/summary.get_total())*100.0
		);
	}

	println!("{tab}+—{type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	let total_spent_msg: String = "Total".to_string();
	println!(
		"{tab}| {:<type_width$} | {:>PRICE_WIDTH$.2} |            |",
		total_spent_msg,
		summary.get_total()
	);
	println!("{tab}+—{type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	println!("");
	println!("");
}

pub fn display_and_accounting_expenses<F>(
	month_data: &MonthlyActivities<Expense>,
	func: &F,
	type_depth: usize
)
-> ActivitySummary
where
	F: Fn(&Expense) -> bool
{
	let shop_column_width =
		std::cmp::max(
			4,
			month_data
				.iter()
				.filter(|e: &&Expense| func(e))
				.map(|e: &Expense| -> usize { e.shop.chars().count() })
				.max()
				.unwrap_or(0)
		);
	let shop_main_divider = std::iter::repeat("—").take(shop_column_width).collect::<String>();
	let shop_mid_divider: String = std::iter::repeat("·").take(shop_column_width).collect::<String>();
	let shop_header = center_string(&"Shop".to_string(), shop_column_width);

	let city_column_width =
		std::cmp::max(
			5,
			month_data
				.iter()
				.filter(|e: &&Expense| func(e))
				.map(|e: &Expense| -> usize { e.city.chars().count() })
				.max()
				.unwrap_or(0)
		);
	let city_main_divider = std::iter::repeat("—").take(city_column_width).collect::<String>();
	let city_mid_divider: String = std::iter::repeat("·").take(city_column_width).collect::<String>();
	let city_header = center_string(&"City".to_string(), city_column_width);

	let type_columns_width = calculate_type_columns_width(month_data, func);
	let type_column_width: usize =
		type_columns_width.iter().sum::<usize>()
		+
		(type_columns_width.len() - 1)*CONCEPT_TYPE_SEPARATOR.len()
		;

	let concept_type_main_divider = std::iter::repeat("—").take(type_column_width).collect::<String>();
	let concept_type_mid_divider: String = std::iter::repeat("·").take(type_column_width).collect::<String>();
	let concept_type_header = center_string(&"Type".to_string(), type_column_width);

	let price_main_divider = std::iter::repeat("—").take(PRICE_WIDTH).collect::<String>();
	let price_mid_divider: String = std::iter::repeat("·").take(PRICE_WIDTH).collect::<String>();
	let price_header = center_string(&"Price".to_string(), PRICE_WIDTH);

	let date_main_divider = std::iter::repeat("—").take(DATE_WIDTH).collect::<String>();
	let date_mid_divider: String = std::iter::repeat("·").take(DATE_WIDTH).collect::<String>();
	let date_header = center_string(&"Date".to_string(), DATE_WIDTH);

	let mut summary = ActivitySummary::new();
	
	let mut first: bool = true;
	let mut size_data: u32 = 0;
	let mut previous_date: date::Date = date::Date { year: 1900, month: date::Month::January, day: 1};
	for (i, Expense {
		day_of_year: d,
		price: pr,
		concepts: cs,
		shop: pl,
		city: ci,
		description: descr
	})
	in month_data.iter().filter(|e| func(e)).enumerate()
	{
		size_data = size_data + 1;

		let types_to_summarize: Vec<String> = cs.iter().take(type_depth).cloned().collect();
		summary.add(types_to_summarize, *pr);

		let expense_type_text = types_text(cs, &type_columns_width, CONCEPT_TYPE_SEPARATOR);
		let place_text = center_string( pl, shop_column_width);
		let city_text = center_string( ci, city_column_width);
		if &previous_date != d {

			if first {
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{shop_main_divider}—+—{city_main_divider}—+");
				println!("    | ID | {date_header} | {price_header} | {concept_type_header} | {shop_header} | {city_header} | Description");
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{shop_main_divider}—+—{city_main_divider}—+");
				first = false;
			}
			else {
				println!("    +————+—{date_mid_divider}—+—{price_mid_divider}—+—{concept_type_mid_divider}—+—{shop_mid_divider}—+—{city_mid_divider}—+");
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
	if size_data > 0 {
		println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{shop_main_divider}—+—{city_main_divider}—+");
		println!("");
	}

	if size_data > 1 {
		display_summary_activity(&summary, &"    ");
	}

	summary
}

pub fn display_and_accounting_incomes<F>(
	month_data: &MonthlyActivities<Income>,
	func: &F,
	type_depth: usize
)
-> ActivitySummary
where
	F: Fn(&Income) -> bool
{
	let place_column_width =
		std::cmp::max(
			5,
			month_data
				.iter()
				.filter(|i: &&Income| func(i))
				.map(|i: &Income| -> usize { i.place.chars().count() })
				.max()
				.unwrap_or(0)
		);
	let place_main_divider = std::iter::repeat("—").take(place_column_width).collect::<String>();
	let place_mid_divider: String = std::iter::repeat("·").take(place_column_width).collect::<String>();
	let place_header = center_string(&"Place".to_string(), place_column_width);

	let from_column_width =
		std::cmp::max(
			5,
			month_data
				.iter()
				.filter(|i: &&Income| func(i))
				.map(|i: &Income| -> usize { i.from.chars().count() })
				.max()
				.unwrap_or(0)
		);
	let from_main_divider = std::iter::repeat("—").take(from_column_width).collect::<String>();
	let from_mid_divider: String = std::iter::repeat("·").take(from_column_width).collect::<String>();
	let from_header = center_string(&"From".to_string(), from_column_width);

	let type_columns_width = calculate_type_columns_width(month_data, func);
	let type_sep = " ; ";
	let type_column_width: usize =
		type_columns_width.iter().sum::<usize>()
		+
		(type_columns_width.len() - 1)*type_sep.len()
		;

	let concept_type_main_divider = std::iter::repeat("—").take(type_column_width).collect::<String>();
	let concept_type_mid_divider: String = std::iter::repeat("·").take(type_column_width).collect::<String>();
	let concept_type_header = center_string(&"Type".to_string(), type_column_width);

	let price_main_divider = std::iter::repeat("—").take(PRICE_WIDTH).collect::<String>();
	let price_mid_divider: String = std::iter::repeat("·").take(PRICE_WIDTH).collect::<String>();
	let price_header = center_string(&"Price".to_string(), PRICE_WIDTH);

	let date_main_divider = std::iter::repeat("—").take(DATE_WIDTH).collect::<String>();
	let date_mid_divider: String = std::iter::repeat("·").take(DATE_WIDTH).collect::<String>();
	let date_header = center_string(&"Date".to_string(), DATE_WIDTH);

	let mut summary = ActivitySummary::new();
	
	let mut first: bool = true;
	let mut size_data: u32 = 0;
	let mut previous_date: date::Date = date::Date { year: 1900, month: date::Month::January, day: 1};
	for (i, Income {
		day_of_year: d,
		price: pr,
		concepts: cs,
		from: fr,
		place: pl,
		description: descr
	})
	in month_data.iter().filter(|e| func(e)).enumerate()
	{
		size_data = size_data + 1;

		let types_to_summarize: Vec<String> = cs.iter().take(type_depth).cloned().collect();
		summary.add(types_to_summarize, *pr);

		let income_type_text = types_text(cs, &type_columns_width, type_sep);
		let place_text = center_string( pl, place_column_width);
		let from_text = center_string( fr, from_column_width);
		if &previous_date != d {

			if first {
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{place_main_divider}—+—{from_main_divider}—+");
				println!("    | ID | {date_header} | {price_header} | {concept_type_header} | {place_header} | {from_header} | Description");
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{place_main_divider}—+—{from_main_divider}—+");
				first = false;
			}
			else {
				println!("    +————+—{date_mid_divider}—+—{price_mid_divider}—+—{concept_type_mid_divider}—+—{place_mid_divider}—+—{from_mid_divider}—+");
			}

			let date_text = center_string(&d.to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {income_type_text} | {place_text} | {from_text} | {descr}");
			previous_date = d.clone();
		}
		else {
			let date_text = center_string(&" ".to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {income_type_text} | {place_text} | {from_text} | {descr}");
		}
	}
	if size_data > 0 {
		println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{place_main_divider}—+—{from_main_divider}—+");
		println!("");
	}

	if size_data > 1 {
		display_summary_activity(&summary, &"    ");
	}

	summary
}

pub struct Cell {
	pub num_times: u32,
	pub total_value: f32,
	pub classifier: String
}

pub fn display_history_summary(
	vec_summary: &Vec<(String, Cell)>,
	first_title: String,
	second_title: String
)
{
	let first_column_width =
	std::cmp::max(
		first_title.len(),
		vec_summary
		.iter()
		.map(|v| -> usize { v.0.len() })
		.max()
		.unwrap_or(0)
	);
	let first_main_divider = std::iter::repeat("—").take(first_column_width).collect::<String>();
	let first_header = center_string(&first_title, first_column_width);

	let second_column_width =
	std::cmp::max(
		second_title.len(),
		vec_summary
		.iter()
		.map(|v| -> usize { v.1.classifier.len() })
		.max()
		.unwrap_or(0)
	);
	let second_main_divider = std::iter::repeat("—").take(second_column_width).collect::<String>();
	let second_header = center_string(&second_title, second_column_width);

	let tab = "    ";

	if second_column_width > 0 {
		println!("{tab}+—{first_main_divider}—+—{second_main_divider}—+—————————————+———————————————————+");
		println!("{tab}| {first_header} | {second_header} | Times found | Total money spent |");
		println!("{tab}+—{first_main_divider}—+—{second_main_divider}—+—————————————+———————————————————+");
	}
	else {
		println!("{tab}+—{first_main_divider}—+—————————————+———————————————————+");
		println!("{tab}| {first_header} | Times found | Total money spent |");
		println!("{tab}+—{first_main_divider}—+—————————————+———————————————————+");
	}
	
	for (thing, Cell {classifier: city, num_times, total_value}) in vec_summary.iter() {
		let thing_text = center_string( thing, first_column_width);
		if second_column_width > 0 {
			let city_text = center_string( city, second_column_width);
			println!("{tab}| {thing_text} | {city_text} | {num_times:>11} | {total_value:>17.2} |");
		}
		else {
			println!("{tab}| {thing_text} | {num_times:>11} | {total_value:>17.2} |");
		}
	}
	if second_column_width > 0 {
		println!("{tab}+—{first_main_divider}—+—{second_main_divider}—+—————————————+———————————————————+");
	}
	else {
		println!("{tab}+—{first_main_divider}—+—————————————+———————————————————+");
	}
}
