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

use crate::date;

use crate::expense::Expense;
use crate::income::Income;
use crate::monthly_activities::MonthlyActivities;
use crate::yearly_activities::YearlyActivities;
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
	let mut yearly_expenses = YearlyActivities::new();
	
	if let Some(file_name) = p.file_name() {
		if let Some(file_str) = file_name.to_str() {
			yearly_expenses.year = file_str[..4].parse::<u32>().unwrap();
		}
	}
	
	let mut monthly_expenses = MonthlyActivities::new();
	let mut previous_month = date::Month::January;
	
	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		let l = line.unwrap();
		if l == "" { continue; }
		
		let e = Expense::from_str(&l).expect("Expected expense");
		
		if e.day_of_year.month == previous_month {
			monthly_expenses.expenses.push(e);
		}
		else {
			if monthly_expenses.expenses.len() > 0 {
				yearly_expenses.activities.push(monthly_expenses);
			}
			
			monthly_expenses = MonthlyActivities {
				month: e.day_of_year.month.clone(),
				expenses : vec![],
				incomes: Vec::new()
			};
			
			previous_month = e.day_of_year.month.clone();
			monthly_expenses.expenses.push(e);
		}
	}
	
	yearly_expenses.activities.push(monthly_expenses);
	return yearly_expenses;
}

pub fn read_income_file(p: &std::path::PathBuf) -> YearlyActivities {
	let mut yearly_income = YearlyActivities::new();
	
	if let Some(file_name) = p.file_name() {
		if let Some(file_str) = file_name.to_str() {
			yearly_income.year = file_str[..4].parse::<u32>().unwrap();
		}
	}
	
	let mut monthly_income = MonthlyActivities::new();
	let mut previous_month = date::Month::January;
	
	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		let l = line.unwrap();
		if l == "" { continue; }
		
		let e = Income::from_str(&l).expect("Expected expense");
		
		if e.day_of_year.month == previous_month {
			monthly_income.incomes.push(e);
		}
		else {
			if monthly_income.incomes.len() > 0 {
				yearly_income.activities.push(monthly_income);
			}
			
			monthly_income = MonthlyActivities {
				month: e.day_of_year.month.clone(),
				expenses : Vec::new(),
				incomes: Vec::new()
			};
			
			previous_month = e.day_of_year.month.clone();
			monthly_income.incomes.push(e);
		}
	}
	
	yearly_income.activities.push(monthly_income);
	return yearly_income;
}

pub fn read_all_activities_data(data_dir: &String) -> AllActivities {
	let mut all_data = AllActivities::new();
	
	{
	let paths = std::fs::read_dir(data_dir.to_owned() + &"expenses").unwrap();
	for path in paths {
		let path = path.unwrap().path();
		
		println!("        Reading '{}'...", path.display());
		
		let r = read_expense_file(&path);
		
		if all_data.min_year > r.year {
			all_data.min_year = r.year;
		}
		if all_data.max_year < r.year {
			all_data.max_year = r.year;
		}
		all_data.merge(r);
	}
	}
	{
	let paths = std::fs::read_dir(data_dir.to_owned() + &"incomes").unwrap();
	for path in paths {
		let path = path.unwrap().path();
		
		println!("        Reading '{}'...", path.display());
		
		let r = read_income_file(&path);
		
		if all_data.min_year > r.year {
			all_data.min_year = r.year;
		}
		if all_data.max_year < r.year {
			all_data.max_year = r.year;
		}
		all_data.merge(r);
	}
	}
	all_data.activities.sort();
	
	all_data
}

fn read_types(data_dir: &String, filename: String) -> Vec<String> {
	let mut types = Vec::new();

	let path = data_dir.to_owned() + &filename;
	let file = std::fs::File::open( path.clone() ).expect("Failed to open file");

	println!("        Reading '{path}'...");

	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		let l = line.unwrap();
		if l == "" { continue; }
		types.push( l.trim().to_string() );
	}
	types
}

pub fn read_expense_types(data_dir: &String) -> Vec<String> {
	read_types(data_dir, "expense_types.txt".to_string())
}

pub fn read_income_types(data_dir: &String) -> Vec<String> {
	read_types(data_dir, "income_types.txt".to_string())
}

/* ------------------------------------------------------------------------- */

pub fn write_all_data(data_dir: &String, all_data: &AllActivities) -> Result<()> {
	for ye in all_data.activities.iter() {
		if !ye.has_changes() { continue; }
		
		{
		let expense_filename = data_dir.to_owned() + &format!("expenses/{}.txt", ye.year).to_string();
		println!("Writing into '{expense_filename}'...");
		let mut expense_file = std::fs::File::create(expense_filename).expect("I wanted to create a file");
		for me in ye.activities.iter() {
			for Expense {
				day_of_year: d,
				price: pr,
				expense_type: et,
				place: pl,
				description: descr
			}
			in me.expenses.iter()
			{
				writeln!(expense_file, "{d} {pr} \"{et}\" \"{pl}\" \"{descr}\"")?;
			}
		}
		}

		{
		let income_filename = data_dir.to_owned() + &format!("incomes/{}.txt", ye.year).to_string();
		println!("Writing into '{income_filename}'...");
		let mut income_file = std::fs::File::create(income_filename).expect("I wanted to create a file");
		for me in ye.activities.iter() {
			for Income {
				day_of_year: d,
				price: pr,
				concept: ct,
				from: fr,
				place: pl,
				description: descr
			}
			in me.incomes.iter()
			{
				writeln!(income_file, "{d} {pr} \"{ct}\" \"{fr}\" \"{pl}\" \"{descr}\"")?;
			}
		}
		}
	}
	if all_data.expense_types.has_changes() {
		let filename = data_dir.to_owned() + &"/expense_types.txt".to_string();
		println!("Writing into '{filename}'...");
		let mut file = std::fs::File::create(filename).expect("I wanted to create a file");
		for et in all_data.expense_types.types.iter() {
			writeln!(file, "{et}")?;
		}
	}
	if all_data.income_types.has_changes() {
		let filename = data_dir.to_owned() + &"/income_types.txt".to_string();
		println!("Writing into '{filename}'...");
		let mut file = std::fs::File::create(filename).expect("I wanted to create a file");
		for et in all_data.income_types.types.iter() {
			writeln!(file, "{et}")?;
		}
	}

	Ok(())
}
