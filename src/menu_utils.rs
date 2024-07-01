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

fn center_string(s: &String, width: usize) -> String {
	let length_s = s.chars().count();
	let left_pad_size = (width - length_s)/2;
    let right_pad_size = width - left_pad_size - length_s;
	
	let left_pad: String = std::iter::repeat(" ").take(left_pad_size).collect::<String>();
	let right_pad: String = std::iter::repeat(" ").take(right_pad_size).collect::<String>();

	format!("{left_pad}{s}{right_pad}")
}

static CONCEPT_TYPE_WIDTH: usize = 4;
static CONCEPT_SUBTYPE_WIDTH: usize = 7;
static PRICE_WIDTH: usize = 8;
static DATE_WIDTH: usize = 17;

pub fn display_summary_activity(summary: &ActivitySummary, pre_tab: &str) {
	let concept_type_width = std::cmp::max(CONCEPT_TYPE_WIDTH, summary.get_width_concept_type());
	let concept_type_main_divider = std::iter::repeat("—").take(concept_type_width).collect::<String>();
	let concept_type_header = center_string(&"Type".to_string(), concept_type_width);

	let concept_subtype_width = std::cmp::max(CONCEPT_SUBTYPE_WIDTH, summary.get_width_concept_subtype());
	let concept_subtype_main_divider = std::iter::repeat("—").take(concept_subtype_width).collect::<String>();
	let concept_subtype_header = center_string(&"Subtype".to_string(), concept_subtype_width);

	let price_main_divider = std::iter::repeat("—").take(PRICE_WIDTH).collect::<String>();
	let price_header = center_string(&"Price".to_string(), PRICE_WIDTH);

	let percentage_width = 10;
	let percentage_main_divider = std::iter::repeat("—").take(percentage_width).collect::<String>();
	let percentage_header = center_string(&"Percentage".to_string(), percentage_width);

	println!("");
	let tab = pre_tab.to_owned() + "    ";
	println!("{tab}+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	println!("{tab}| {concept_type_header} | {concept_subtype_header} | {price_header} | {percentage_header} |");
	println!("{tab}+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	for ((activity_type, activity_subtype), value) in summary.iter_summary() {
		println!(
			"{tab}| {:<concept_type_width$} | {:<concept_subtype_width$} | {:>PRICE_WIDTH$.2} | {:>9.2}% |",
			activity_type,
			activity_subtype,
			value,
			(value/summary.get_total())*100.0
		);
	}
	
	println!("{tab}+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	let total_spent_msg: String = "Total".to_string();
	println!(
		"{tab}| {:<concept_type_width$} | {:<concept_subtype_width$} | {:>PRICE_WIDTH$.2} |            |",
		total_spent_msg,
		"".to_string(),
		summary.get_total()
	);
	println!("{tab}+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	println!("");
	println!("");
}

pub fn display_and_accounting_expenses<F>(month_data: &MonthlyActivities<Expense>, func: &F)
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
				.map(|i: &Expense| -> usize { i.shop.len() })
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
				.map(|i: &Expense| -> usize { i.city.len() })
				.max()
				.unwrap_or(0)
		);
	let city_main_divider = std::iter::repeat("—").take(city_column_width).collect::<String>();
	let city_mid_divider: String = std::iter::repeat("·").take(city_column_width).collect::<String>();
	let city_header = center_string(&"City".to_string(), city_column_width);

	let concept_type_column_width =
		std::cmp::max(
			CONCEPT_TYPE_WIDTH,
			month_data
				.iter()
				.filter(|i: &&Expense| func(i))
				.map(|i: &Expense| -> usize { i.concept.len() })
				.max()
				.unwrap_or(0)
		);
	let concept_type_main_divider = std::iter::repeat("—").take(concept_type_column_width).collect::<String>();
	let concept_type_mid_divider: String = std::iter::repeat("·").take(concept_type_column_width).collect::<String>();
	let concept_type_header = center_string(&"Type".to_string(), concept_type_column_width);

	let concept_subtype_column_width =
		std::cmp::max(
			CONCEPT_SUBTYPE_WIDTH,
			month_data
				.iter()
				.filter(|i: &&Expense| func(i))
				.map(|i: &Expense| -> usize { i.sub_concept.len() })
				.max()
				.unwrap_or(0)
		);
	let concept_subtype_main_divider = std::iter::repeat("—").take(concept_subtype_column_width).collect::<String>();
	let concept_subtype_mid_divider: String = std::iter::repeat("·").take(concept_subtype_column_width).collect::<String>();
	let concept_subtype_header = center_string(&"Subtype".to_string(), concept_subtype_column_width);

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
		sub_concept: sc,
		shop: pl,
		city: ci,
		description: descr
	})
	in month_data.iter().filter(|e| func(e)).enumerate()
	{
		some_data = true;
		summary.add(et.clone(), sc.clone(), *pr);

		let expense_type_text = center_string( et, concept_type_column_width);
		let expense_subtype_text = center_string( sc, concept_subtype_column_width);
		let place_text = center_string( pl, shop_column_width);
		let city_text = center_string( ci, city_column_width);
		if &previous_date != d {

			if first {
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{shop_main_divider}—+—{city_main_divider}—+");
				println!("    | ID | {date_header} | {price_header} | {concept_type_header} | {concept_subtype_header} | {shop_header} | {city_header} | Description");
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{shop_main_divider}—+—{city_main_divider}—+");
				first = false;
			}
			else {
				println!("    +————+—{date_mid_divider}—+—{price_mid_divider}—+—{concept_type_mid_divider}—+—{concept_subtype_mid_divider}—+—{shop_mid_divider}—+—{city_mid_divider}—+");
			}

			let date_text = center_string(&d.to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {expense_type_text} | {expense_subtype_text} | {place_text} | {city_text} | {descr}");
			previous_date = d.clone();
		}
		else {
			let date_text = center_string(&" ".to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {expense_type_text} | {expense_subtype_text} | {place_text} | {city_text} | {descr}");
		}
	}
	if some_data {
		println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{shop_main_divider}—+—{city_main_divider}—+");
	}

	if some_data {
		display_summary_activity(&summary, &"    ");
	}

	summary
}

pub fn display_and_accounting_incomes<F>(month_data: &MonthlyActivities<Income>, func: &F)
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
				.map(|i: &Income| -> usize { i.place.len() })
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
				.map(|i: &Income| -> usize { i.from.len() })
				.max()
				.unwrap_or(0)
		);
	let from_main_divider = std::iter::repeat("—").take(from_column_width).collect::<String>();
	let from_mid_divider: String = std::iter::repeat("·").take(from_column_width).collect::<String>();
	let from_header = center_string(&"City".to_string(), from_column_width);

	let concept_type_column_width =
		std::cmp::max(
			CONCEPT_TYPE_WIDTH,
			month_data
				.iter()
				.filter(|i: &&Income| func(i))
				.map(|i: &Income| -> usize { i.concept.len() })
				.max()
				.unwrap_or(0)
		);
	let concept_type_main_divider = std::iter::repeat("—").take(concept_type_column_width).collect::<String>();
	let concept_type_mid_divider: String = std::iter::repeat("·").take(concept_type_column_width).collect::<String>();
	let concept_type_header = center_string(&"Type".to_string(), concept_type_column_width);

	let concept_subtype_column_width =
		std::cmp::max(
			CONCEPT_SUBTYPE_WIDTH,
			month_data
				.iter()
				.filter(|i: &&Income| func(i))
				.map(|i: &Income| -> usize { i.sub_concept.len() })
				.max()
				.unwrap_or(0)
		);
	let concept_subtype_main_divider = std::iter::repeat("—").take(concept_subtype_column_width).collect::<String>();
	let concept_subtype_mid_divider: String = std::iter::repeat("·").take(concept_subtype_column_width).collect::<String>();
	let concept_subtype_header = center_string(&"Subtype".to_string(), concept_subtype_column_width);

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
	for (i, Income {
		day_of_year: d,
		price: pr,
		concept: et,
		sub_concept: sc,
		from: fr,
		place: pl,
		description: descr
	})
	in month_data.iter().filter(|e| func(e)).enumerate()
	{
		some_data = true;
		summary.add(et.clone(), sc.clone(), *pr);

		let income_type_text = center_string( et, concept_type_column_width);
		let income_subtype_text = center_string( sc, concept_subtype_column_width);
		let place_text = center_string( pl, place_column_width);
		let from_text = center_string( fr, from_column_width);
		if &previous_date != d {

			if first {
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{place_main_divider}—+—{from_main_divider}—+");
				println!("    | ID | {date_header} | {price_header} | {concept_type_header} | {concept_subtype_header} | {place_header} | {from_header} | Description");
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{place_main_divider}—+—{from_main_divider}—+");
				first = false;
			}
			else {
				println!("    +————+—{date_mid_divider}—+—{price_mid_divider}—+—{concept_type_mid_divider}—+—{concept_subtype_mid_divider}—+—{place_mid_divider}—+—{from_mid_divider}—+");
			}

			let date_text = center_string(&d.to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {income_type_text} | {income_subtype_text} | {place_text} | {from_text} | {descr}");
			previous_date = d.clone();
		}
		else {
			let date_text = center_string(&" ".to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {income_type_text} | {income_subtype_text} | {place_text} | {from_text} | {descr}");
		}
	}
	if some_data {
		println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{concept_type_main_divider}—+—{concept_subtype_main_divider}—+—{place_main_divider}—+—{from_main_divider}—+");
	}

	if some_data {
		display_summary_activity(&summary, &"    ");
	}

	summary
}

pub struct Cell {
	pub num_times: u32,
	pub total_value: f32,
	pub city: String
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
		.map(|v| -> usize { v.1.city.len() })
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
	
	for (thing, Cell {city, num_times, total_value}) in vec_summary.iter() {
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
