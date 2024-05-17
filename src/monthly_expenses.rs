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
				println!("This date already existed, but it's Ok!");
				self.expenses.insert(idx, exp);
			},
			Err(idx) => {
				println!("This date did not exist before, and it's Ok!");
				self.expenses.insert(idx, exp);
			}
		}
	}

}
