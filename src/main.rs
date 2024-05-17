extern crate serde;
extern crate serde_json;

use std::io::Read;

mod date;
mod expense;
mod monthly_expenses;
mod yearly_expenses;
mod all_expenses;
mod io;
mod expense_types;

type Expense = expense::Expense;
type ExpenseTypes = expense_types::ExpenseTypes;
type MonthlyExpenses = monthly_expenses::MonthlyExpenses;
type YearlyExpenses = yearly_expenses::YearlyExpenses;
type AllExpenses = all_expenses::AllExpenses;

/* ------------------------------------------------------------------------- */

fn read_option<F: Fn()>(f: F, min_valid: u32, max_valid: u32) -> u32 {
	f();
	
	let mut option = io::read_input_string();
	
	while option == "" {  option = io::read_input_string(); }
	let mut option_int: u32 = option.parse().unwrap();
	
	while !(min_valid <= option_int && option_int <= max_valid) {
		f();
		
		option = io::read_input_string();
		while option == "" {  option = io::read_input_string(); }
		option_int = option.parse().unwrap();
	}
	
	option_int
}

/* ------------------------------------------------------------------------- */

fn print_expense_types_all(all_expense_data: &AllExpenses) {
	println!("");
	for (i, expense_type) in all_expense_data.expense_types.types.iter().enumerate() {
		println!("    {i}: {expense_type}");
	}
	println!("");
}

fn add_expense_type(all_expense_data: &mut AllExpenses) {
	println!("Enter the new type of expense:");
	let new_expense = io::read_input_string();

	if all_expense_data.expense_types.exists_expense_type(&new_expense) {
		println!("Expense type '{new_expense}' already exists.")
	}
	else {
		all_expense_data.expense_types.add_element(new_expense);
	}
}

fn rename_expense_type(all_expense_data: &mut AllExpenses) {
	println!("Enter the type of expense to rename:");
	let old_expense = io::read_input_string();

	let idx_old_expense = all_expense_data.expense_types.position_expense_type(&old_expense);
	
	if let Some(idx_old) = idx_old_expense {
		println!("Enter the new type of expense:");
		let new_expense = io::read_input_string();

		all_expense_data.expense_types.replace_element(idx_old, new_expense.clone());

		// replace the old expense type throughout the entire list of expenses
		for ye in all_expense_data.expenses.iter_mut() {
			ye.set_changes(true);
			for me in ye.expenses.iter_mut() {
				for e in me.expenses.iter_mut().filter(|e| e.expense_type == old_expense) {
					e.expense_type = new_expense.clone();
				}
			}
		}
	}
	else {
		println!("Expense type '{old_expense}' does not exist.")
	}
}

fn remove_expense_type(all_expense_data: &mut AllExpenses) {
	println!("Enter the type of expense to remove:");
	let expense_to_remove = io::read_input_string();

	let idx_expense =
		all_expense_data.expense_types.position_expense_type(&expense_to_remove);

	if let Some(idx) = idx_expense {
		all_expense_data.expense_types.remove_element(idx);
	}
	else {
		println!("Expense type to remove '{expense_to_remove}' does not exist.");
	}
}

fn print_expense_type_menu() {
	println!("Query and edit the expense types:");
	println!("");
	println!("    1. Show all expense types");
	println!("    2. Add a new expense type");
	println!("    3. Rename a specific expense type");
	println!("    4. Remove an expense type");
	println!("    0. Leave");
}

fn expense_types_menu(all_expense_data: &mut AllExpenses) {
	let print_function = print_expense_type_menu;
	let min_option = 0;
	let max_option = 4;
	
	let mut option = read_option(print_function, min_option, max_option);
	while option != 0 {
		if option == 1 {
			print_expense_types_all(&all_expense_data);
		}
		else if option == 2 {
			add_expense_type(all_expense_data);
		}
		else if option == 3 {
			rename_expense_type(all_expense_data);
		}
		else if option == 4 {
			remove_expense_type(all_expense_data);
		}
		
		option = read_option(print_function, min_option, max_option);
	}
	
	println!("Goodbye!");
}

/* ------------------------------------------------------------------------- */

fn print_expense_data_month(month_data: &MonthlyExpenses) {
	let mut accounting: std::collections::HashMap<String, f32> = std::collections::HashMap::new();
	let mut total_spent: f32 = 0.0;
	let mut total_income: f32 = 0.0;

	let month = &month_data.month;
	println!("    Month {month} ({})", month_data.expenses.len());

	{
	let entries_str = format!("{}", month_data.expenses.len()).to_string();
	let dashes_number: String = std::iter::repeat('-').take( entries_str.len() ).collect();

	let month_name_str = format!("{}", month_data.month).to_string();
	let dashes_month: String = std::iter::repeat('-').take(month_name_str.len()).collect();
	println!("    ------{}--{}-", dashes_month, dashes_number);
	println!("");
	}

	for (i, Expense {
		day_of_year: d,
		price: pr,
		expense_type: et,
		place: pl,
		description: descr
	})
	in month_data.expenses.iter().enumerate()
	{
		if et == "Income" || et == "Ingressos" {
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
	//             00000000011111111112222222222
	//             12345678901234567890123456789
	println!("{tab}Expense type       Total   Percentage");
	println!("{tab}-------------------------------------");
	for (expense_type, value) in accounting.iter() {
		println!("{tab}{expense_type:<15}   {value:>6.2}      {:>6.2}%", (value/total_spent)*100.0);
	}
	
	println!("{tab}-------------------------------------");
	let total_spent_msg: String = "Total spent".to_string();
	println!("{tab}{total_spent_msg:<15}   {total_spent:>6.2}");
	println!("{tab}{:<15}   {total_income:>6.2}", "Income");
	println!("{tab}-------------------------------------");
	let balance_msg: String = "Balance".to_string();
	println!("{tab}{balance_msg:<15}   {:>6.2}", total_spent + total_income);
	println!("");
	println!("");
}

fn print_expense_data_year(year_data: &YearlyExpenses) {
	println!("Data from year: {}", year_data.year);
	println!("====================");
	
	let mut total_entries = 0;
	for month_data in year_data.expenses.iter() {
		total_entries += month_data.expenses.len();
	}
	
	println!("    Found {} entries", total_entries);
	for month_data in year_data.expenses.iter() {
		print_expense_data_month(month_data);
	}
}

fn print_expense_data_all(all_expense_data: &AllExpenses) {
	for year_expense in all_expense_data.expenses.iter() {
		print_expense_data_year(&year_expense);
	}
}

fn print_expense_data_year_user(all_expense_data: &AllExpenses) {
	println!("What year do you want to see?");
	let year_int: u32 = io::read_input_string().parse().unwrap();
	
	let res = all_expense_data.get_year(&year_int);
	if let Some(year) = res {
		print_expense_data_year(year);
	}
	else {
		println!("Year {year_int} not found!");
	};
}

fn print_expense_data_month_user(all_expense_data: &AllExpenses) {
	println!("What year and month do you want to see? Year -> Month");
	let year_int: u32 = io::read_input_string().parse().unwrap();
	
	let month_str = io::read_input_string();
	let month = month_str.parse::<date::Month>().expect(
		format!("String '{month_str}' not valid for a month").as_str()
	);
	
	let res = all_expense_data.get_month(&year_int, &month);
	if let Some(&ref month_data) = res {
		print_expense_data_month(&month_data);
	}
	else {
		println!("Month '{month}' does not exist in year '{year_int}'.");
	}
	
}

fn add_new_expense(all_expense_data: &mut AllExpenses) {
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
	println!("Expense Type:");
	let expense_type = io::read_input_string();
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

fn expenses_menu(all_expense_data: &mut AllExpenses) {
	let print_function = print_expenses_menu;
	let min_option = 0;
	let max_option = 4;
	
	let mut option = read_option(print_function, min_option, max_option);
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
		
		option = read_option(print_function, min_option, max_option);
	}
	
}

/* ------------------------------------------------------------------------- */

fn print_main_menu() {
	println!("What menu do you want to access?");
	println!("");
	println!("    1. Expenses menu");
	println!("    2. Expenses types menu");
	println!("    3. Save all data");
	println!("    0. Leave");
}

fn main_menu(all_expense_data: &mut AllExpenses, data_dir: &String) {
	let print_function = print_main_menu;
	let min_option = 0;
	let max_option = 3;
	
	let mut option = read_option(print_function, min_option, max_option);
	while option != 0 {
		if option == 1 {
			expenses_menu(all_expense_data);
		}
		else if option == 2 {
			expense_types_menu(all_expense_data);
		}
		else if option == 3 {
			io::write_all_expense_data(&data_dir, all_expense_data).expect("Could not write data");
			for ye in all_expense_data.expenses.iter_mut() {
				ye.set_changes(false);
			}
			all_expense_data.expense_types.set_changes(false);
		}
		
		option = read_option(print_function, min_option, max_option);
	}
	
	println!("Goodbye!");
}

/* ------------------------------------------------------------------------- */

#[derive(serde::Serialize, serde::Deserialize)]
struct ProjectData {
	pub base_path: String
}

fn main() {
	println!("Welcome to the expenses manager!");
	println!("");

	let mut file = std::fs::File::open("project_configuration.json").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();

	let json: ProjectData = serde_json::from_str(&data).unwrap();
	let data_dir = json.base_path;
	
	println!("Reading data from directory '{data_dir}'...");
	println!("    Reading expense data...");
	let mut all_expense_data = io::read_all_expense_data(&data_dir);
	
	println!("    Reading expense types...");
	all_expense_data.expense_types = ExpenseTypes::new_vec(
		io::read_expense_types(&data_dir)
	);

	println!("");
	println!("");
	println!("");
	println!("");
	main_menu(&mut all_expense_data, &data_dir);
	
	io::write_all_expense_data(&data_dir, &all_expense_data).expect("Could not write data");

}
