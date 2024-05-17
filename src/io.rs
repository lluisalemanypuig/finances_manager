use crate::date::Month;

use crate::expense::Expense;
use crate::monthly_expenses::MonthlyExpenses;
use crate::yearly_expenses::YearlyExpenses;
use crate::all_expenses::AllExpenses;
use crate::expense_types::ExpenseTypes;

use std::io::{BufRead, Write, Result};
use std::str::FromStr;

pub fn read_input_string() -> String {
	let mut s = String::new();
	let stdin = std::io::stdin();
	stdin.read_line(&mut s).expect("I was expecting standard input");
	s.trim().to_string()
}

/* ------------------------------------------------------------------------- */

pub fn read_expense_file(p: &std::path::PathBuf) -> YearlyExpenses {
	let mut yearly_expenses = YearlyExpenses::new();
	
	if let Some(file_name) = p.file_name() {
		if let Some(file_str) = file_name.to_str() {
			yearly_expenses.year = file_str[..4].parse::<u32>().unwrap();
		}
	}
	
	let mut monthly_expenses = MonthlyExpenses {
		month: Month::January,
		expenses : Vec::new()
	};
	let mut previous_month = Month::January;
	
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
				yearly_expenses.expenses.push(monthly_expenses);
			}
			
			monthly_expenses = MonthlyExpenses {
				month: e.day_of_year.month.clone(),
				expenses : vec![]
			};
			
			previous_month = e.day_of_year.month.clone();
			monthly_expenses.expenses.push(e);
		}
	}
	
	yearly_expenses.expenses.push(monthly_expenses);
	return yearly_expenses;
}

pub fn read_all_expense_data(data_dir: &String) -> AllExpenses {
	let mut all_expense_data = AllExpenses {
		min_year: 9999,
		max_year: 0,
		expense_types: ExpenseTypes::new(),
		expenses: Vec::new()
	};
	
	// all files
	let paths = std::fs::read_dir(data_dir.to_owned() + &"/expenses").unwrap();
	for path in paths {
		let path = path.unwrap().path();
		
		println!("        Reading '{}'...", path.display());
		
		let r = read_expense_file(&path);
		
		if all_expense_data.min_year > r.year {
			all_expense_data.min_year = r.year;
		}
		if all_expense_data.max_year < r.year {
			all_expense_data.max_year = r.year;
		}
		all_expense_data.expenses.push(r);
	}
	all_expense_data.expenses.sort();
	
	all_expense_data
}

pub fn read_expense_types(data_dir: &String) -> Vec<String> {
	let mut expense_types = Vec::new();

	let path = data_dir.to_owned() + &"/expense_types.txt";
	let file = std::fs::File::open( path.clone() ).expect("Failed to open file");

	println!("        Reading '{path}'...");

	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		let l = line.unwrap();
		if l == "" { continue; }
		
		expense_types.push( l.trim().to_string() );
	}

	expense_types
}

/* ------------------------------------------------------------------------- */

pub fn write_all_expense_data(data_dir: &String, all_expense_data: &AllExpenses) -> Result<()> {
	for ye in all_expense_data.expenses.iter() {
		if !ye.has_changes() { continue; }
		
		let filename = data_dir.to_owned() + &format!("/expenses/{}.txt", ye.year).to_string();
		println!("Writing into '{filename}'...");
		
		let mut file = std::fs::File::create(filename).expect("I wanted to create a file");
		
		for me in ye.expenses.iter() {
			
			for Expense {
				day_of_year: d,
				price: pr,
				expense_type: et,
				place: pl,
				description: descr
			}
			in me.expenses.iter()
			{
				writeln!(file, "{d} {pr} \"{et}\" \"{pl}\" \"{descr}\"")?;
			}
		}
	}
	if all_expense_data.expense_types.has_changes() {
		let filename = data_dir.to_owned() + &"/expense_types.txt".to_string();
		println!("Writing into '{filename}'...");
		
		let mut file = std::fs::File::create(filename).expect("I wanted to create a file");
		
		for et in all_expense_data.expense_types.types.iter() {
			writeln!(file, "{et}")?;
		}
	}

	Ok(())
}
