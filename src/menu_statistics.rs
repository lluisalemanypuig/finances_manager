use crate::io;
use crate::menu_utils;

use crate::expense;
use crate::all_expenses;

type Expense = expense::Expense;
type AllExpenses = all_expenses::AllExpenses;

fn statistics_by_place(all_data: &AllExpenses) {
	let place: String = io::read_input_string();

	let mut accounting: std::collections::BTreeMap<String, f32> = std::collections::BTreeMap::new();
	let mut total_spent: f32 = 0.0;
	let mut total_income: f32 = 0.0;

	for year_data in all_data.expenses.iter() {
		println!("Data from year: {}", year_data.year);
		println!("====================");

		for month_data in year_data.expenses.iter() {

			{
			let month = &month_data.month;
			println!("	{month} ({})", month_data.expenses.len());
		
			let entries_str = format!("{}", month_data.expenses.len()).to_string();
			let dashes_number: String = std::iter::repeat('-').take( entries_str.len() ).collect();
		
			let month_name_str = format!("{}", month_data.month).to_string();
			let dashes_month: String = std::iter::repeat('-').take(month_name_str.len()).collect();
			println!("	{}--{}-", dashes_month, dashes_number);
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
				if pl != &place { continue; }

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
				println!("        {i:>2}: {d_string:>17} | {pr:>6.2} | {et:>15} | {pl:>25} | {descr}");
			}
			println!("");
		}
	}

	println!("");
	let tab = "            ";
	println!("{tab}{:<15}	{:>6}	{:>10}", "Expense type", "Total", "Percentage");
	println!("{tab}---------------------------------------");
	for (expense_type, value) in accounting.iter() {
		println!("{tab}{:<15}	{:>6.2}	{:>9.2}%", expense_type, value, (value/total_spent)*100.0);
	}
	
	println!("{tab}---------------------------------------");
	let total_spent_msg: String = "Total spent".to_string();
	println!("{tab}{:<15}	{:>6.2}", total_spent_msg, total_spent);
	println!("{tab}{:<15}	{:>6.2}", all_data.expense_types.income_name, total_income);
	println!("{tab}---------------------------------------");
	let balance_msg: String = "Balance".to_string();
	println!("{tab}{:<15}	{:>6.2}", balance_msg, total_spent + total_income);
	println!("");
	println!("");

}


fn print_statistics_menu() {
	println!("Perform statistics:");
	println!("");
	println!("    1. By place");
	println!("    0. Leave");
}

pub fn menu(all_data: &AllExpenses) {
	let print_function = print_statistics_menu;
	let min_option = 0;
	let max_option = 1;

	let mut option = menu_utils::read_option(print_function, min_option, max_option);
	while option != 0 {
		
		match option {
			1 => statistics_by_place(&all_data),
			_ => println!("Nothing to do..."),
		}
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
}