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

use crate::date;

use crate::expense::Expense;
use crate::income::Income;
use crate::monthly_activities::MonthlyActivities;
use crate::yearly_activities::YearlyActivities;
use crate::concept_types;
use crate::concept_types::ConceptTypes;
use crate::all_activities::AllActivities;

use std::io::{BufRead, Write, Result};
use std::str::FromStr;

pub fn read_input_string() -> String {
	let mut s = String::new();
	let stdin = std::io::stdin();
	stdin.read_line(&mut s).expect("I was expecting standard input");
	s.trim().to_string()
}

pub fn read_string_or_empty() -> Option<String> {
	let str = read_input_string();
	if str == "".to_string() { return None; }
	Some(str)
}

pub fn read_string() -> String {
	loop {
		if let Some(str) = read_string_or_empty() {
			break str;
		}
	}
}

pub trait Numeric: FromStr {}
impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for usize {}
impl Numeric for f32 {}
impl Numeric for f64 {}

pub fn read_num_or_empty<T>() -> Option<T> where T: Numeric {
	loop {
		if let Some(str) = read_string_or_empty() {
			if let Ok(value) = str.parse::<T>() {
				break Some(value);
			}
		}
		else {
			break None;
		}
	}
}

pub fn read_num<T: FromStr>() -> T where T: Numeric {
	loop {
		if let Ok(value) = read_string().parse::<T>() {
			return value;
		}
	}
}

pub trait Integral: Numeric {}
impl Integral for u8 {}
impl Integral for u16 {}
impl Integral for u32 {}
impl Integral for u64 {}
impl Integral for usize {}

pub fn read_int_or_empty<T: FromStr>() -> Option<T> where T: Integral {
	read_num_or_empty::<T>()
}

pub fn read_int<T: FromStr>() -> T where T: Integral {
	read_num::<T>()
}

pub trait Decimal: Numeric {}
impl Decimal for f32 {}
impl Decimal for f64 {}

pub fn read_float_or_empty<T: FromStr>() -> Option<T> where T: Decimal {
	read_num_or_empty::<T>()
}

pub fn read_float<T: FromStr>() -> T where T: Decimal {
	read_num::<T>()
}

/* ------------------------------------------------------------------------- */

pub fn read_from_options_or_empty(options: &Vec<String>) -> Option<String> {
	loop {
		if let Some(str) = read_string_or_empty() {
			if str == "?".to_string() {
				for opt in options.iter() {
					println!("    {opt}");
				}
				println!("");
			}
			else if options.contains(&str) {
				return Some(str);
			}
		}
		else {
			return None;
		}
	}
}

pub fn read_from_options(options: &Vec<String>) -> String {
	loop {
		if let Some(s) = read_from_options_or_empty(options) {
			return s;
		}
	}
}

/* ------------------------------------------------------------------------- */

pub fn read_correct_month() -> Option<date::Month> {
	loop {
		match read_string_or_empty() {
			Some(str) => {
				let month_res = str.parse::<date::Month>();
				if let Ok(m) = month_res {
					return Some(m);
				}
			},
			None => {}
		}
	}
}

/* ------------------------------------------------------------------------- */

pub fn read_expense_file(p: &std::path::PathBuf) -> YearlyActivities {
	let mut this_file_year = 45;
	
	if let Some(file_name) = p.file_name() {
		if let Some(file_str) = file_name.to_str() {
			this_file_year = file_str[..4].parse::<u32>().unwrap();
		}
	}
	
	let mut yearly_expenses = YearlyActivities::new_year_changes(this_file_year, false);
	let mut monthly_expenses = MonthlyActivities::<Expense>::new();
	let mut previous_month = date::Month::January;
	
	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		let l = line.unwrap();
		if l == "" { continue; }
		
		let e = Expense::from_str(&l).expect("Expected expense");
		
		if e.day_of_year.month == previous_month {
			monthly_expenses.push(e);
		}
		else {
			if monthly_expenses.size() > 0 {
				yearly_expenses.get_expenses_mut().push(monthly_expenses);
			}
			
			monthly_expenses = MonthlyActivities::<Expense>::new_month(&e.day_of_year.month);
			
			previous_month = e.day_of_year.month.clone();
			monthly_expenses.push(e);
		}
	}
	
	yearly_expenses.get_expenses_mut().push(monthly_expenses);
	return yearly_expenses;
}

pub fn read_income_file(p: &std::path::PathBuf) -> YearlyActivities {
	
	let mut this_file_year = 45;

	if let Some(file_name) = p.file_name() {
		if let Some(file_str) = file_name.to_str() {
			this_file_year = file_str[..4].parse::<u32>().unwrap()
		}
	}
	
	let mut yearly_incomes = YearlyActivities::new_year_changes(this_file_year, false);

	let mut monthly_income = MonthlyActivities::<Income>::new();
	let mut previous_month = date::Month::January;
	
	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		let l = line.unwrap();
		if l == "" { continue; }
		
		let i = Income::from_str(&l).expect("Expected income");
		
		if i.day_of_year.month == previous_month {
			monthly_income.push(i);
		}
		else {
			if monthly_income.size() > 0 {
				yearly_incomes.get_incomes_mut().push(monthly_income);
			}
			
			monthly_income = MonthlyActivities::<Income>::new_month(
				&i.day_of_year.month.clone()
			);
			
			previous_month = i.day_of_year.month.clone();
			monthly_income.push(i);
		}
	}
	
	yearly_incomes.get_incomes_mut().push(monthly_income);
	return yearly_incomes;
}

pub fn read_all_activities_data(data_dir: &String) -> AllActivities {
	let mut all_data = AllActivities::new();
	
	let expense_path = std::fs::read_dir(data_dir.to_owned() + &"expenses").unwrap();
	for path in expense_path {
		let path = path.unwrap().path();
		println!("        Reading '{}'...", path.display());
		let r = read_expense_file(&path);
		all_data.merge(r);
	}
	let income_path = std::fs::read_dir(data_dir.to_owned() + &"incomes").unwrap();
	for path in income_path {
		let path = path.unwrap().path();
		println!("        Reading '{}'...", path.display());
		let r = read_income_file(&path);
		all_data.merge(r);
	}
	all_data.get_activities_mut().sort();
	
	all_data
}

fn read_types(data_dir: &String, filename: String, concept_types: &mut ConceptTypes) {
	let path = data_dir.to_owned() + &filename;
	let file = std::fs::File::open( path.clone() ).expect("Failed to open file");

	println!("        Reading '{path}'...");

	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		let l = line.unwrap();
		if l == "" { continue; }
		
		let values: Vec<&str> = l
			.split_terminator(":")
			.map(|s| s.trim())
			.collect();

		let concept_type = values[0].to_string();
		concept_types.add_concept(concept_type.clone());
		let concept_subtypes: Vec<String> = values[1..].iter().map(|s| s.to_string()).collect();
		concept_types.set_subconcept(concept_type, concept_subtypes);
	}
}

pub fn read_expense_types(data_dir: &String, all_data: &mut AllActivities) {
	read_types(
		data_dir, 
		"expense_types.txt".to_string(),
		all_data.get_expense_concepts_mut()
	);
}

pub fn read_income_types(data_dir: &String, all_data: &mut AllActivities) {
	read_types(
		data_dir, 
		"income_types.txt".to_string(),
		all_data.get_income_concepts_mut()
	);
}

/* ------------------------------------------------------------------------- */

fn write_concept_types(data_dir: &String, filename: String, iter: concept_types::Iter)
-> Result<()>
{
	let filename = data_dir.to_owned() + &filename;
	println!("Writing into '{filename}'...");
	let mut file = std::fs::File::create(filename).expect("I wanted to create a file");
	for (et, subtypes) in iter {
		write!(file, "{et}")?;
		for s in subtypes.iter() {
			write!(file, " : {s}")?;
		}
		writeln!(file, "")?;
	}
	Ok(())
}

pub fn write_all_data(data_dir: &String, all_data: &AllActivities) -> Result<()> {
	for ye in all_data.iter_activities() {
		
		if ye.get_expenses().has_changes() {
			let expense_filename =
				data_dir.to_owned() +
				&format!("expenses/{}.txt", ye.get_year()).to_string();

			println!("Writing into '{expense_filename}'...");
			let mut expense_file =
				std::fs::File::create(expense_filename)
					.expect("I wanted to create an output file for the expenses, but couldn't");
			
			for me in ye.iter_expenses() {
				for Expense {
					day_of_year: d,
					price: pr,
					concept: c,
					sub_concept: sc,
					shop: pl,
					city: ci,
					description: descr
				}
				in me.get_activities().iter()
				{
					writeln!(expense_file, "\"{d}\"\t\"{pr}\"\t\"{c}\"\t\"{sc}\"\t\"{pl}\"\t\"{ci}\"\t\"{descr}\"")?;
				}
			}
		}

		if ye.get_incomes().has_changes() {
			let income_filename =
				data_dir.to_owned() +
				&format!("incomes/{}.txt", ye.get_year()).to_string();

			println!("Writing into '{income_filename}'...");
			let mut income_file =
				std::fs::File::create(income_filename)
				.expect("I wanted to create a file");

			for me in ye.iter_incomes() {
				for Income {
					day_of_year: d,
					price: pr,
					concept: c,
					sub_concept: sc,
					from: fr,
					place: pl,
					description: descr
				}
				in me.get_activities().iter()
				{
					writeln!(income_file, "\"{d}\"\t\"{pr}\"\t\"{c}\"\t\"{sc}\"\t\"{fr}\"\t\"{pl}\"\t\"{descr}\"")?;
				}
			}
		}
	}
	if all_data.get_expense_concepts().has_changes() {
		let filename = "expense_types.txt".to_string();
		println!("Writing into '{filename}'...");
		write_concept_types(data_dir, filename, all_data.iter_expense_subconcepts())?;
	}
	if all_data.get_income_concepts().has_changes() {
		let filename = "income_types.txt".to_string();
		println!("Writing into '{filename}'...");
		write_concept_types(data_dir, filename, all_data.iter_income_subconcepts())?;
	}

	Ok(())
}
