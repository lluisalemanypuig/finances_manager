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
	{
	let month = &month_data.month;
	println!("    {month} ({})", month_data.expenses.len());

	let entries_str = format!("{}", month_data.expenses.len()).to_string();
	let dashes_number: String = std::iter::repeat('-').take( entries_str.len() ).collect();

	let month_name_str = format!("{}", month_data.month).to_string();
	let dashes_month: String = std::iter::repeat('-').take(month_name_str.len()).collect();
	println!("    {}--{}-", dashes_month, dashes_number);
	println!("");
	}

	let mut accounting: std::collections::BTreeMap<String, f32> = std::collections::BTreeMap::new();
	let mut total_spent: f32 = 0.0;
	let mut total_income: f32 = 0.0;

	for (i, Expense {
		day_of_year: d,
		price: pr,
		expense_type: et,
		place: pl,
		description: descr
	})
	in month_data.expenses.iter().enumerate()
	{
		if et == &all_data.expense_types.income_name {
			total_income += pr;
		}
		else {
			total_spent += pr;
			match accounting.get_mut(et) {
				Some(value) => {
					*value += *pr;
				},
				None => {
					accounting.insert(et.clone(), *pr);
				}
			}
		}

		let d_string = d.to_string();
		println!("        {i:>2}: {d_string:>17} | {pr:>6.2} | {et:>15} | {pl:>21} | {descr}");
		
	}

	println!("");
	let tab = "            ";
	println!("{tab}{:<15}    {:>6}    {:>10}", "Expense type", "Total", "Percentage");
	println!("{tab}---------------------------------------");
	for (expense_type, value) in accounting.iter() {
		println!("{tab}{:<15}    {:>6.2}    {:>9.2}%", expense_type, value, (value/total_spent)*100.0);
	}
	
	println!("{tab}---------------------------------------");
	let total_spent_msg: String = "Total spent".to_string();
	println!("{tab}{:<15}    {:>6.2}", total_spent_msg, total_spent);
	println!("{tab}{:<15}    {:>6.2}", all_data.expense_types.income_name, total_income);
	println!("{tab}---------------------------------------");
	let balance_msg: String = "Balance".to_string();
	println!("{tab}{:<15}    {:>6.2}", balance_msg, total_spent + total_income);
	println!("");
	println!("");
}

fn print_expense_data_year(all_data: &AllExpenses, year_data: &YearlyExpenses) {
	println!("Data from year: {}", year_data.year);
	println!("====================");
	
	let mut total_entries = 0;
	for month_data in year_data.expenses.iter() {
		total_entries += month_data.expenses.len();
	}
	
	println!("    Found {} entries", total_entries);
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
	let year_int: u32 = io::read_input_string().parse().unwrap();
	
	let res = all_data.get_year(&year_int);
	if let Some(year) = res {
		print_expense_data_year(all_data, year);
	}
	else {
		println!("Year {year_int} not found!");
	};
}

fn print_expense_data_month_user(all_data: &AllExpenses) {
	println!("What year and month do you want to see? Year -> Month");
	let year_int: u32 = io::read_input_string().parse().unwrap();
	
	let month_str = io::read_input_string();
	let month = month_str.parse::<date::Month>().expect(
		format!("String '{month_str}' not valid for a month").as_str()
	);
	
	let res = all_data.get_month(&year_int, &month);
	if let Some(&ref month_data) = res {
		print_expense_data_month(all_data, &month_data);
	}
	else {
		println!("Month '{month}' does not exist in year '{year_int}'.");
	}
}

fn add_new_expense(all_expense_data: &mut AllExpenses) {
	println!("Expense Type:");
	let expense_type = io::read_input_string();
	if !all_expense_data.expense_types.exists_expense_type(&expense_type) {
		println!("Expense type '{expense_type}' is not valid.");
		return;
	}

	println!("Year:");
	let year = io::read_input_string().parse().unwrap();
	println!("Month:");
	let month_str = io::read_input_string();
	let month: date::Month = month_str.parse::<date::Month>().expect(
		format!("String '{month_str}' not valid for a month").as_str()
	);

	let year_data = all_expense_data.add_year_mut(&year);
	let month_data = year_data.add_month_mut(&month);

	println!("Day:");
	let day: u8 = io::read_input_string().parse().unwrap();
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

fn print_expenses_menu() {
	println!("Query and edit the expenses:");
	println!("");
	println!("    1. Show all current data");
	println!("    2. Show data of a specific year");
	println!("    3. Show data of a specific month");
	println!("    4. Add another expense");
	println!("    0. Leave");
}

pub fn menu(all_expense_data: &mut AllExpenses) {
	let print_function = print_expenses_menu;
	let min_option = 0;
	let max_option = 4;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		if option == 1 {
			print_expense_data_all(&all_expense_data);
		}
		else if option == 2 {
			print_expense_data_year_user(&all_expense_data);
		}
		else if option == 3 {
			print_expense_data_month_user(&all_expense_data);
		}
		else if option == 4 {
			add_new_expense(all_expense_data);
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
	
}
