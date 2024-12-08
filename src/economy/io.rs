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

use crate::economy::all_activities::AllActivities;
use crate::economy::expense::Expense;
use crate::economy::income::Income;
use crate::economy::monthly_activities::MonthlyActivities;
use crate::economy::yearly_activities::YearlyActivities;

use crate::time::date;

use std::io::{BufRead, Result, Write};
use std::str::FromStr;

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
		if l == "" {
			continue;
		}

		let e = Expense::from_str(&l).expect("Expected expense");

		if e.day_of_year.month == previous_month {
			monthly_expenses.push(e);
		} else {
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
		if l == "" {
			continue;
		}

		let i = Income::from_str(&l).expect("Expected income");

		if i.day_of_year.month == previous_month {
			monthly_income.push(i);
		} else {
			if monthly_income.size() > 0 {
				yearly_incomes.get_incomes_mut().push(monthly_income);
			}

			monthly_income = MonthlyActivities::<Income>::new_month(&i.day_of_year.month.clone());

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

pub fn write_all_data(data_dir: &String, all_data: &AllActivities) -> Result<()> {
	for ye in all_data.iter_activities() {
		if ye.get_expenses().has_changes() {
			let expense_filename =
				data_dir.to_owned() + &format!("expenses/{}.txt", ye.get_year()).to_string();

			println!("Writing into '{expense_filename}'...");
			let mut expense_file = std::fs::File::create(expense_filename)
				.expect("I wanted to create an output file for the expenses, but couldn't");

			for me in ye.iter_expenses() {
				for Expense {
					day_of_year: d,
					price: pr,
					concepts: cs,
					shop: pl,
					city: ci,
					description: descr,
				} in me.get_activities().iter()
				{
					let concept_list = cs.join(";");
					writeln!(
						expense_file,
						"\"{d}\"\t\"{pr}\"\t\"{concept_list}\"\t\"{pl}\"\t\"{ci}\"\t\"{descr}\""
					)?;
				}
			}
		}

		if ye.get_incomes().has_changes() {
			let income_filename =
				data_dir.to_owned() + &format!("incomes/{}.txt", ye.get_year()).to_string();

			println!("Writing into '{income_filename}'...");
			let mut income_file =
				std::fs::File::create(income_filename).expect("I wanted to create a file");

			for me in ye.iter_incomes() {
				for Income {
					day_of_year: d,
					price: pr,
					concepts: cs,
					from: fr,
					place: pl,
					description: descr,
				} in me.get_activities().iter()
				{
					let concept_list = cs.join(";");
					writeln!(
						income_file,
						"\"{d}\"\t\"{pr}\"\t\"{concept_list}\"\t\"{fr}\"\t\"{pl}\"\t\"{descr}\""
					)?;
				}
			}
		}
	}
	Ok(())
}
