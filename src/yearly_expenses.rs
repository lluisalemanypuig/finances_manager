use crate::date::Month;
use crate::monthly_expenses::MonthlyExpenses;

#[derive(Debug)]
pub struct YearlyExpenses {
	changes: bool,

	pub year: u32,
	pub expenses: Vec<MonthlyExpenses>
}

impl Eq for YearlyExpenses {}

impl PartialEq for YearlyExpenses {
	fn eq(&self, other: &Self) -> bool {
		self.year == other.year
	}
}
impl Ord for YearlyExpenses {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.year.cmp(&other.year)
	}
}
impl PartialOrd for YearlyExpenses {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq<u32> for YearlyExpenses {
	fn eq(&self, y: &u32) -> bool {
		self.year == *y
	}
}
impl PartialOrd<u32> for YearlyExpenses {
	fn partial_cmp(&self, y: &u32) -> Option<std::cmp::Ordering> {
		Some(self.year.cmp(y))
	}
}
impl PartialEq<YearlyExpenses> for u32 {
	fn eq(&self, other: &YearlyExpenses) -> bool {
		*self == other.year
	}
}
impl PartialOrd<YearlyExpenses> for u32 {
	fn partial_cmp(&self, other: &YearlyExpenses) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&other.year))
	}
}

impl YearlyExpenses {
	pub fn new() -> YearlyExpenses {
		YearlyExpenses {
			changes: false,
			year: 0,
			expenses: Vec::new()
		}
	}
	pub fn new_year(y: &u32, changes: bool) -> YearlyExpenses {
		YearlyExpenses {
			year: *y,
			expenses: Vec::new(),
			changes
		}
	}

	pub fn as_ref(&self) -> &YearlyExpenses { self }
	pub fn as_mut(&mut self) -> &mut YearlyExpenses { self }

	pub fn has_changes(&self) -> bool { self.changes }
	pub fn set_changes(&mut self, c: bool) {
		self.changes = c;
	}

	pub fn has_month(&self, m: &Month) -> bool {
		let res = self.expenses.binary_search_by(|e| e.month.cmp(&m));
		match res {
			Ok(_) => true,
			Err(_) => false
		}
	}

	#[duplicate::duplicate_item(
		method           convert   reference(type);
		[get_month]      [as_ref]  [& type]       ;
		[get_month_mut]  [as_mut]  [&mut type]    ;
	)]
	pub fn method(self: reference([Self]), m: &Month) -> Option<reference([MonthlyExpenses])> {
		let res = self.expenses.binary_search_by(|e| e.month.cmp(&m));
		match res {
			Ok(idx) => Some(self.expenses[idx].convert()),
			Err(_) => None
		}
	}

	pub fn add_month(&mut self, m: &Month) -> &mut MonthlyExpenses {
		let res = self.expenses.binary_search_by(|e| e.month.cmp(&m));
		match res {
			Ok(pos) => {
				// month already exists
				&mut self.expenses[pos]
			},
			Err(pos) => {
				// month does not exist
				self.expenses.insert(pos, 
					MonthlyExpenses {
						month: m.clone(),
						expenses: Vec::new()
					}
				);
				&mut self.expenses[pos]
			}
		}
	}

}
