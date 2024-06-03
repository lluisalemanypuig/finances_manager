use crate::io;

use crate::date;
use crate::expense;
use crate::expense_types;
use crate::monthly_expenses;
use crate::all_expenses;
use crate::expense_summary;

type Expense = expense::Expense;
type ExpenseTypes = expense_types::ExpenseTypes;
type MonthlyExpenses = monthly_expenses::MonthlyExpenses;
type AllExpenses = all_expenses::AllExpenses;
type ExpenseSummary = expense_summary::ExpenseSummary;

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

static EXPENSE_TYPE_WIDTH: usize = 15;
static PRICE_WIDTH: usize = 8;
static DATE_WIDTH: usize = 17;

pub fn display_expense_summary(summary: &ExpenseSummary, all_data: &AllExpenses, pre_tab: &str) {
	let expense_type_main_divider = std::iter::repeat("—").take(EXPENSE_TYPE_WIDTH).collect::<String>();
	let expense_type_header = center_string(&"Expense type".to_string(), EXPENSE_TYPE_WIDTH);

	let price_main_divider = std::iter::repeat("—").take(PRICE_WIDTH).collect::<String>();
	let price_header = center_string(&"Price".to_string(), PRICE_WIDTH);

	let percentage_width = 10;
	let percentage_main_divider = std::iter::repeat("—").take(percentage_width).collect::<String>();
	let percentage_header = center_string(&"Percentage".to_string(), percentage_width);

	println!("");
	let tab = pre_tab.to_owned() + "    ";
	println!("{tab}+—{expense_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	println!("{tab}| {expense_type_header} | {price_header} | {percentage_header} |");
	println!("{tab}+—{expense_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	for (expense_type, value) in summary.expense_to_price.iter() {
		println!("{tab}| {:<EXPENSE_TYPE_WIDTH$} | {:>PRICE_WIDTH$.2} | {:>9.2}% |", expense_type, value, (value/summary.total_spent)*100.0);
	}
	
	println!("{tab}+—{expense_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	let total_spent_msg: String = "Total spent".to_string();
	println!("{tab}| {:<EXPENSE_TYPE_WIDTH$} | {:>PRICE_WIDTH$.2} |            |", total_spent_msg, summary.total_spent);
	println!("{tab}| {:<EXPENSE_TYPE_WIDTH$} | {:>PRICE_WIDTH$.2} |            |", all_data.expense_types.income_name, summary.total_income);
	println!("{tab}+—{expense_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	let balance_msg: String = "Balance".to_string();
	println!("{tab}| {:<EXPENSE_TYPE_WIDTH$} | {:>PRICE_WIDTH$.2} |            |", balance_msg, summary.total_spent - summary.total_income);
	println!("{tab}+—{expense_type_main_divider}—+—{price_main_divider}—+—{percentage_main_divider}—+");
	println!("");
	println!("");
}

pub fn display_and_accounting<F: Fn(&Expense) -> bool>(
	all_data: &AllExpenses,
	month_data: &MonthlyExpenses,
	func: F
)
-> ExpenseSummary
{
	let place_width =
		std::cmp::max(
			5,
			month_data.expenses
			.iter()
			.filter(|e: &&expense::Expense| func(e))
			.fold(0, |max, val| if val.place.len() > max { val.place.len() } else { max })
		);
	let place_main_divider = std::iter::repeat("—").take(place_width).collect::<String>();
	let place_mid_divider: String = std::iter::repeat("·").take(place_width).collect::<String>();
	let place_header = center_string(&"Place".to_string(), place_width);

	let expense_type_main_divider = std::iter::repeat("—").take(EXPENSE_TYPE_WIDTH).collect::<String>();
	let expense_type_mid_divider: String = std::iter::repeat("·").take(EXPENSE_TYPE_WIDTH).collect::<String>();
	let expense_type_header = center_string(&"Expense type".to_string(), EXPENSE_TYPE_WIDTH);

	let price_main_divider = std::iter::repeat("—").take(PRICE_WIDTH).collect::<String>();
	let price_mid_divider: String = std::iter::repeat("·").take(PRICE_WIDTH).collect::<String>();
	let price_header = center_string(&"Price".to_string(), PRICE_WIDTH);

	let date_main_divider = std::iter::repeat("—").take(DATE_WIDTH).collect::<String>();
	let date_mid_divider: String = std::iter::repeat("·").take(DATE_WIDTH).collect::<String>();
	let date_header = center_string(&"Date".to_string(), DATE_WIDTH);

	let mut summary: ExpenseSummary = ExpenseSummary::new();
	
	let mut first: bool = true;
	let mut some_data: bool = false;
	let mut previous_date: date::Date = date::Date { year: 1900, month: date::Month::January, day: 1};
	for (i, Expense {
		day_of_year: d,
		price: pr,
		expense_type: et,
		place: pl,
		description: descr
	})
	in month_data.expenses.iter().filter(|e| func(e)).enumerate()
	{
		some_data = true;
		if et == &all_data.expense_types.income_name {
			summary.total_income += pr;
		}
		else {
			summary.total_spent += pr;
			match summary.expense_to_price.get_mut(et) {
				Some(value) => {
					*value += *pr;
				},
				None => {
					summary.expense_to_price.insert(et.clone(), *pr);
				}
			}
		}

		let expense_type_text = center_string( et, EXPENSE_TYPE_WIDTH);
		let place_text = center_string( pl, place_width);
		if &previous_date != d {

			if first {
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{expense_type_main_divider}—+—{place_main_divider}—+");
				println!("    | ID | {date_header} | {price_header} | {expense_type_header} | {place_header} | Description");
				println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{expense_type_main_divider}—+—{place_main_divider}—+");
				first = false;
			}
			else {
				println!("    +————+—{date_mid_divider}—+—{price_mid_divider}—+—{expense_type_mid_divider}—+—{place_mid_divider}—+");
			}

			let date_text = center_string(&d.to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {expense_type_text} | {place_text} | {descr}");
			previous_date = d.clone();
		}
		else {
			let date_text = center_string(&" ".to_string(), DATE_WIDTH);
			println!("    | {i:>2} | {date_text} | {pr:>PRICE_WIDTH$.2} | {expense_type_text} | {place_text} | {descr}");
		}
	}
	if some_data {
		println!("    +————+—{date_main_divider}—+—{price_main_divider}—+—{expense_type_main_divider}—+—{place_main_divider}—+");
	}

	if some_data {
		display_expense_summary(&summary, all_data, &"    ");
	}

	summary
}

pub fn display_full_summary(
	vec_summary: &Vec<(String, (u32,f32))>,
	title: String
)
{
	let title_width =
	std::cmp::max(
		title.len(),
		vec_summary
		.iter()
		.fold(0, |max, val| if val.0.len() > max { val.0.len() } else { max })
	);
	let title_main_divider = std::iter::repeat("—").take(title_width).collect::<String>();
	let title_header = center_string(&title, title_width);

	let tab = "    ";
	println!("{tab}+—{title_main_divider}—+—————————————+———————————————————+");
	println!("{tab}| {title_header} | Times found | Total money spent |");
	println!("{tab}+—{title_main_divider}—+—————————————+———————————————————+");
	for (thing, (num_times, total_value)) in vec_summary.iter() {
		let thing_text = center_string( thing, title_width);
		println!("{tab}| {thing_text} | {num_times:>11} | {total_value:>17.2} |");
	}
	println!("{tab}+—{title_main_divider}—+—————————————+———————————————————+");
}

pub fn read_correct_expense_type(expense_types: &ExpenseTypes) -> Option<String> {
	loop {
		if let Some(str) = io::read_string_or_empty() {
			if expense_types.is_expense_type_ok(&str) {
				return Some(str);
			}
		}
		else {
			return None;
		}
	}
}