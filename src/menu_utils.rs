use crate::io;

use crate::expense;
use crate::monthly_expenses;
use crate::all_expenses;

type Expense = expense::Expense;
type MonthlyExpenses = monthly_expenses::MonthlyExpenses;
type AllExpenses = all_expenses::AllExpenses;

pub fn read_option<F: Fn()>(f: F, min_valid: u32, max_valid: u32) -> u32 {
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

pub fn display_and_accounting<F: Fn(&Expense) -> bool>(
	all_data: &AllExpenses,
	month_data: &MonthlyExpenses,
	func: F
)
{
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
	in month_data.expenses.iter().filter(|e| func(e)).enumerate()
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
		println!("        {i:>2}: {d_string:>17} | {pr:>6.2} | {et:>15} | {pl:>25} | {descr}");
		
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
