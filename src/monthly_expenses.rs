use crate::expense::Expense;
use crate::date::Month;

#[derive(Debug)]
pub struct MonthlyExpenses {
	pub month: Month,
	pub expenses: Vec<Expense>
}

impl MonthlyExpenses {

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

	pub fn get_expense(&self, i: usize) -> &Expense {
		&self.expenses[i]
	}

	pub fn get_expense_mut(&mut self, i: usize) -> &mut Expense {
		&mut self.expenses[i]
	}

	pub fn remove_expense(&mut self, i: usize) {
		self.expenses.remove(i);
	}

	pub fn num_expenses(&self) -> usize {
		self.expenses.len()
	}
}
