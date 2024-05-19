use crate::io;
use crate::menu_utils;

use crate::all_expenses;

type AllExpenses = all_expenses::AllExpenses;

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

pub fn menu(all_expense_data: &mut AllExpenses) {
	let print_function = print_expense_type_menu;
	let min_option = 0;
	let max_option = 4;
	
	let mut option = menu_utils::read_option(print_function, min_option, max_option);
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
		
		option = menu_utils::read_option(print_function, min_option, max_option);
	}
	
	println!("Goodbye!");
}
