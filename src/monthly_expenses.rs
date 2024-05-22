extern crate duplicate;

use crate::expense::Expense;
use crate::date::Month;

#[derive(Debug)]
pub struct MonthlyExpenses {
	pub month: Month,
	pub expenses: Vec<Expense>
}

impl MonthlyExpenses {
	pub fn as_ref(&self) -> &MonthlyExpenses { self }
	pub fn as_mut(&mut self) -> &mut MonthlyExpenses { self }

	pub fn add_expense(&mut self, exp: Expense) {
		let pos = self.expenses.binary_search(&exp);
		match pos {
			Ok(idx) => {
				self.expenses.insert(idx, exp);
			},
			Err(idx) => {
				self.expenses.insert(idx, exp);
			}
		}
	}

	#[duplicate::duplicate_item(
		method             convert   reference(type);
		[get_expense]      [as_ref]  [& type]       ;
		[get_expense_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), i: usize) -> reference([Expense]) {
		self.expenses[i].convert()
	}

	pub fn remove_expense(&mut self, i: usize) {
		self.expenses.remove(i);
	}

	pub fn num_expenses(&self) -> usize {
		self.expenses.len()
	}
}
